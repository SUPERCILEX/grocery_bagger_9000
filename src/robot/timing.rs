use std::{cmp::min, time::Duration};

use bevy::{ecs::schedule::ShouldRun, math::const_vec3, prelude::*};
use bevy_prototype_lyon::prelude::DrawMode;
use bevy_rapier3d::prelude::{Collider, CollisionGroups, RapierContext};
use bevy_tweening::{AnimationSystem, Animator, TweenCompleted};

use crate::{
    animations::{AnimationComponentsBundle, AnimationEvent, GameSpeed, Target},
    bags::{
        BagMarker, BagReplacementDetectionSystems, BagSize, BAG_FLOOR_COLLIDER_GROUP,
        BAG_WALLS_COLLIDER_GROUP,
    },
    conveyor_belt::BeltMovementSystems,
    levels::{LevelFinished, LevelMarker, ScoringSystems},
    nominos::{
        Nomino, NominoBundle, NominoColor, PiecePlaced, PieceSystems, Selected,
        NOMINO_COLLIDER_GROUP,
    },
    robot::{spawn::RobotMarker, RobotOptions},
};

const PLACEMENT_TTL: Duration = Duration::from_secs(6);
const MAX_TTL: Duration = Duration::from_secs(10);
const INVALID_PLACEMENT_GROUPS: CollisionGroups = CollisionGroups {
    memberships: BAG_FLOOR_COLLIDER_GROUP.memberships
        | BAG_WALLS_COLLIDER_GROUP.memberships
        | NOMINO_COLLIDER_GROUP.memberships,
    filters: BAG_FLOOR_COLLIDER_GROUP.filters
        | BAG_WALLS_COLLIDER_GROUP.filters
        | NOMINO_COLLIDER_GROUP.filters,
};

pub struct RobotTimingPlugin;

impl Plugin for RobotTimingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            remove_robot_on_disable.with_run_criteria(|options: Res<RobotOptions>| {
                if options.is_changed() && !options.enabled {
                    ShouldRun::Yes
                } else {
                    ShouldRun::No
                }
            }),
        );

        app.add_system(
            accumulate_left_over_time
                .with_run_criteria(run_if_robot_is_enabled)
                .after(PieceSystems),
        );
        app.add_system(
            place_piece
                .with_run_criteria(run_if_robot_is_enabled)
                .after(PieceSystems)
                .after(ScoringSystems)
                .after(BagReplacementDetectionSystems)
                .after(BeltMovementSystems)
                .after(AnimationSystem::AnimationUpdate),
        );
        app.add_system(
            show_target_placement
                .with_run_criteria(run_if_robot_is_enabled)
                .after(accumulate_left_over_time)
                .after(place_piece),
        );
    }
}

#[derive(Component)]
pub struct RobotTiming {
    ttl: Timer,
    continue_trying: bool,
}

#[derive(Component)]
pub struct RobotTargetMarker;

#[derive(Component)]
struct IndicatorPieceMarker;

impl Default for RobotTiming {
    fn default() -> Self {
        Self {
            ttl: Timer::new(PLACEMENT_TTL, false),
            continue_trying: false,
        }
    }
}

fn run_if_robot_is_enabled(options: Res<RobotOptions>) -> ShouldRun {
    if options.enabled {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn remove_robot_on_disable(
    mut commands: Commands,
    robot_entities: Query<Entity, With<IndicatorPieceMarker>>,
) {
    for id in robot_entities.iter() {
        commands.entity(id).despawn_recursive();
    }
}

fn accumulate_left_over_time(
    mut piece_placements: EventReader<PiecePlaced>,
    mut timing: Query<&mut RobotTiming, With<RobotMarker>>,
) {
    if piece_placements.iter().count() == 0 {
        return;
    }

    if let Ok(mut robot) = timing.get_single_mut() {
        robot.continue_trying = false;

        let ttl = &mut robot.ttl;
        ttl.set_duration(min(MAX_TTL, ttl.duration() - ttl.elapsed() + PLACEMENT_TTL));
        ttl.reset();
    }
}

fn show_target_placement(
    mut commands: Commands,
    mut indicator_piece: Query<(&mut DrawMode, &mut Transform), With<IndicatorPieceMarker>>,
    mut spawned: Local<Option<(Entity, Entity)>>,
    mut level_finished: EventReader<LevelFinished>,
    mut completed_animations: EventReader<TweenCompleted>,
    timing: Query<&RobotTiming, With<RobotMarker>>,
    bags: Query<
        (Entity, &GlobalTransform, &BagSize),
        (
            With<BagMarker>,
            Without<RobotTargetMarker>,
            Without<Animator<Transform>>,
        ),
    >,
    selected_piece: Query<(), With<Selected>>,
    target_piece: Query<
        (
            Entity,
            &GlobalTransform,
            Option<&Target<Transform>>,
            &Collider,
            &Nomino,
            &NominoColor,
            ChangeTrackers<RobotTargetMarker>,
        ),
        (With<RobotTargetMarker>, Without<BagMarker>),
    >,
    rapier_context: Res<RapierContext>,
) {
    if level_finished.iter().count() > 0 {
        *spawned = None;
        return;
    }

    let spawned_copy = *spawned;
    let mut maybe_despawn = || {
        if let Some((_, indicator)) = *spawned {
            commands.entity(indicator).despawn_recursive();
            *spawned = None;
        }
    };

    let robot = if let Ok(r) = timing.get_single() {
        r
    } else {
        maybe_despawn();
        return;
    };
    let (target_id, piece_position, target, collider, nomino, color, target_changes) =
        if let Ok(p) = target_piece.get_single() {
            p
        } else {
            maybe_despawn();
            return;
        };
    let mut piece_position = *piece_position;
    if let Some(target) = target {
        piece_position.rotation = target.rotation;
    }

    let render_color = || {
        let alpha = robot.ttl.elapsed().div_duration_f32(robot.ttl.duration());
        let mut color = color.render();
        color.set_a(alpha);
        color
    };

    for TweenCompleted { user_data, .. } in completed_animations.iter() {
        let flags = (AnimationEvent::COMPLETED | AnimationEvent::BAG).bits();
        if *user_data & flags == flags {
            // TODO remove after stageless
            // Force a recompute on the next frame. We need to do this because the bags
            // query filters by animation and so the bag won't show up.
            maybe_despawn();
        }
    }

    if !target_changes.is_changed() && spawned_copy.is_some() {
        if let Some((_, indicator)) = spawned_copy {
            let mut colors = indicator_piece.get_mut(indicator).unwrap().0;
            if let DrawMode::Outlined {
                ref mut fill_mode, ..
            } = *colors
            {
                fill_mode.color = render_color();
            }
        }
        return;
    }

    if let Some((_, position, _)) = find_robot_piece_placement(
        bags,
        selected_piece,
        &piece_position,
        collider,
        &rapier_context,
    ) {
        if let Some((target, indicator)) = spawned_copy && target == target_id {
            let (mut colors, mut transform) = indicator_piece.get_mut(indicator).unwrap();

            *transform = piece_position.with_translation(position).into();
            if let DrawMode::Outlined {
                ref mut fill_mode, ..
            } = *colors
            {
                fill_mode.color = render_color();
            }
        } else {
            maybe_despawn();
            *spawned = Some((
                target_id,
                commands
                    .spawn_bundle(NominoBundle::new(
                        piece_position.with_translation(position).into(),
                        *nomino,
                        *color,
                        render_color(),
                    ))
                    .insert(LevelMarker)
                    .insert(IndicatorPieceMarker)
                    .id(),
            ));
        }
    } else {
        maybe_despawn();
    }
}

fn place_piece(
    mut commands: Commands,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut timing: Query<&mut RobotTiming, With<RobotMarker>>,
    bags: Query<
        (Entity, &GlobalTransform, &BagSize),
        (
            With<BagMarker>,
            Without<RobotTargetMarker>,
            Without<Animator<Transform>>,
        ),
    >,
    selected_piece: Query<(), With<Selected>>,
    mut target_piece: Query<
        (
            Entity,
            &GlobalTransform,
            &mut Transform,
            Option<&Target<Transform>>,
            &Collider,
        ),
        (With<RobotTargetMarker>, Without<BagMarker>),
    >,
    mut piece_placements: EventWriter<PiecePlaced>,
    rapier_context: Res<RapierContext>,
) {
    let mut robot = if let Ok(r) = timing.get_single_mut() {
        r
    } else {
        return;
    };
    robot.ttl.tick(time.delta().mul_f32(**game_speed));
    if !robot.continue_trying && !robot.ttl.just_finished() {
        return;
    }
    robot.continue_trying = true;

    let (piece, piece_position, mut local_piece_position, target, collider) =
        if let Ok(p) = target_piece.get_single_mut() {
            p
        } else {
            return;
        };
    let mut piece_position = *piece_position;
    if let Some(target) = target {
        piece_position.rotation = target.rotation;
    }

    if let Some((bag, mut position, bag_position)) = find_robot_piece_placement(
        bags,
        selected_piece,
        &piece_position,
        collider,
        &rapier_context,
    ) {
        position.z = piece_position.translation.z;
        local_piece_position.translation = position - bag_position;
        piece_placements.send(PiecePlaced { piece, bag });

        commands.entity(bag).add_child(piece);
        commands
            .entity(piece)
            .remove_bundle::<AnimationComponentsBundle<Transform>>();
    }
}

fn find_robot_piece_placement(
    bags: Query<
        (Entity, &GlobalTransform, &BagSize),
        (
            With<BagMarker>,
            Without<RobotTargetMarker>,
            Without<Animator<Transform>>,
        ),
    >,
    ignore: Query<(), With<Selected>>,
    target_piece: &GlobalTransform,
    collider: &Collider,
    rapier_context: &RapierContext,
) -> Option<(Entity, Vec3, Vec3)> {
    let max_rows = bags.iter().map(|b| b.2.height()).sum();
    for row in 0..max_rows {
        for (bag, bag_coords, bag_size) in bags.iter() {
            if row >= bag_size.height() {
                continue;
            }

            let block_origin =
                bag_coords.translation - bag_size.origin() + const_vec3!([0.5, 0.5, 0.]);
            for i in 0..bag_size.width() {
                let position = block_origin + Vec3::new(f32::from(i), f32::from(row), 0.);
                let intersects = rapier_context.intersection_with_shape(
                    position,
                    target_piece.rotation,
                    collider,
                    INVALID_PLACEMENT_GROUPS.into(),
                    Some(&|id| !ignore.contains(id)),
                );
                if intersects.is_some() {
                    continue;
                }

                return Some((bag, position, bag_coords.translation));
            }
        }
    }
    None
}
