use bevy::{math::const_vec3, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_tweening::TweenCompleted;
use smallvec::SmallVec;

use crate::{
    animations,
    animations::{AnimationEvent, GameSpeed},
    bags,
    bags::{
        spawn::{BagLidMarker, BagMarker},
        BagSpawner, BAG_LID_COLLIDER_GROUP, RADIUS,
    },
    conveyor_belt,
    levels::CurrentLevel,
    nominos::{NominoMarker, PiecePlaced, NOMINO_COLLIDER_GROUP},
};

pub struct BagReplacementPlugin;

impl Plugin for BagReplacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BagFilled>();

        app.add_system_to_stage(CoreStage::PostUpdate, detect_filled_bags);
        app.add_system(replace_full_bags);
        app.add_system_to_stage(CoreStage::PostUpdate, despawn_bags);
    }
}

#[derive(Deref)]
struct BagFilled(Entity);

#[derive(Component, Deref, DerefMut)]
pub struct BagPieces(pub SmallVec<[Entity; conveyor_belt::MAX_NUM_PIECES]>);

fn detect_filled_bags(
    mut piece_placements: EventReader<PiecePlaced>,
    mut bags: Query<(&GlobalTransform, &mut BagPieces), With<BagMarker>>,
    mut filled_events: EventWriter<BagFilled>,
    rapier_context: Res<RapierContext>,
    piece_colliders: Query<(&GlobalTransform, &Collider), With<NominoMarker>>,
    lid_collider_bag: Query<&Parent, With<BagLidMarker>>,
) {
    for PiecePlaced { piece, bag } in piece_placements.iter() {
        let (bag_coords, mut bag_pieces) = bags.get_mut(*bag).unwrap();
        bag_pieces.push(*piece);

        {
            let (transform, collider) = piece_colliders.get(*piece).unwrap();
            let bag_overflowing = rapier_context
                .intersection_with_shape(
                    transform.translation,
                    transform.rotation,
                    collider,
                    BAG_LID_COLLIDER_GROUP.into(),
                    Some(&|entity| **lid_collider_bag.get(entity).unwrap() == *bag),
                )
                .is_some();
            if bag_overflowing {
                filled_events.send(BagFilled(*bag));
                continue;
            }
        }

        let mut bag_coords = *bag_coords;
        bag_coords.translation += const_vec3!([0.5 - bags::RADIUS, bags::RADIUS - 0.5, 0.]);

        let mut top_row_full = true;
        for i in 0..6 {
            let mut intersection = false;
            rapier_context.intersections_with_point(
                bag_coords.translation + Vec3::new(i as f32, 0., 0.),
                NOMINO_COLLIDER_GROUP.into(),
                None,
                |_| {
                    intersection = true;
                    false
                },
            );

            if !intersection {
                top_row_full = false;
                break;
            }
        }
        if top_row_full {
            filled_events.send(BagFilled(*bag));
        }
    }
}

fn replace_full_bags(
    mut commands: Commands,
    mut filled_events: EventReader<BagFilled>,
    current_level: Res<CurrentLevel>,
    game_speed: Res<GameSpeed>,
    bag_positions: Query<&Transform, With<BagMarker>>,
    bag_pieces: Query<&BagPieces, With<BagMarker>>,
    mut piece_positions: Query<
        (&GlobalTransform, &mut Transform),
        (With<NominoMarker>, Without<BagMarker>),
    >,
) {
    for filled_bag in filled_events.iter() {
        let current_bag_position = bag_positions.get(**filled_bag).unwrap();
        let new_bag_start = {
            let mut p = *current_bag_position;
            p.scale = Vec3::ZERO;
            p
        };

        commands
            .entity(current_level.root.unwrap())
            .with_children(|parent| {
                parent
                    .spawn_bag_into(new_bag_start)
                    .insert(animations::bag_enter(
                        new_bag_start,
                        *current_bag_position,
                        &game_speed,
                    ));
            });

        let exit_bag_position = {
            let mut p = *current_bag_position;
            p.translation.y = -(RADIUS + 0.5);
            p
        };
        commands.entity(**filled_bag).insert(animations::bag_exit(
            *current_bag_position,
            exit_bag_position,
            &game_speed,
        ));

        // TODO remove after https://github.com/dimforge/bevy_rapier/issues/172
        for piece in &**bag_pieces.get(**filled_bag).unwrap() {
            commands.entity(**filled_bag).add_child(*piece);

            let bag_global = current_bag_position.translation;
            let (piece_global, mut piece_local) = piece_positions.get_mut(*piece).unwrap();
            piece_local.translation = piece_global.translation - bag_global;
        }
    }
}

fn despawn_bags(mut commands: Commands, mut completed_animations: EventReader<TweenCompleted>) {
    for TweenCompleted { entity, user_data } in completed_animations.iter() {
        if *user_data & AnimationEvent::BAG_OFF_SCREEN.bits() != 0 {
            commands.entity(*entity).despawn_recursive();
        }
    }
}
