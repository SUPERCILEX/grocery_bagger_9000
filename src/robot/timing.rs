use std::time::Duration;

use bevy::{math::const_vec3, prelude::*};
use bevy_rapier3d::prelude::{Collider, CollisionGroups, RapierContext};
use bevy_tweening::Animator;

use crate::{
    animations::{GameSpeed, RedoableAnimationBundle},
    bags::{BagMarker, BagSize, BAG_FLOOR_COLLIDER_GROUP, BAG_WALLS_COLLIDER_GROUP},
    nominos::{PiecePlaced, PieceSystems, NOMINO_COLLIDER_GROUP},
    robot::spawn::RobotMarker,
};

const PLACEMENT_TTL: Duration = Duration::from_secs(5);
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
        app.add_system(accumulate_left_over_time.after(PieceSystems));
        app.add_system(place_piece.before(PieceSystems));
    }
}

#[derive(Component)]
pub struct RobotTargetMarker;

#[derive(Component)]
pub struct RobotTiming {
    ttl: Timer,
    continue_trying: bool,
}

impl Default for RobotTiming {
    fn default() -> Self {
        Self {
            ttl: Timer::new(PLACEMENT_TTL, false),
            continue_trying: false,
        }
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
        ttl.set_duration(ttl.duration() - ttl.elapsed() + PLACEMENT_TTL);
        ttl.reset();
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
    mut target_piece: Query<
        (Entity, &mut GlobalTransform, &Collider),
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

    let (piece, mut piece_position, collider) = if let Ok(p) = target_piece.get_single_mut() {
        p
    } else {
        return;
    };

    if let Some((bag, mut position)) =
        find_robot_piece_placement(bags, &piece_position, collider, &rapier_context)
    {
        position.z = piece_position.translation.z;
        piece_position.translation = position;
        piece_placements.send(PiecePlaced { piece, bag });

        commands
            .entity(piece)
            .remove_bundle::<RedoableAnimationBundle<Transform>>();
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
    target_piece: &GlobalTransform,
    collider: &Collider,
    rapier_context: &RapierContext,
) -> Option<(Entity, Vec3)> {
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
                    None,
                );
                if intersects.is_some() {
                    continue;
                }

                return Some((bag, position));
            }
        }
    }
    None
}
