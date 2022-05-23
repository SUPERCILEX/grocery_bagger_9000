use bevy::{math::const_vec3, prelude::*, transform::TransformSystem::TransformPropagate};
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    animations,
    animations::GameSpeed,
    bags,
    bags::{
        spawn::{BagContainerMarker, BagLidMarker, BagMarker},
        BagSpawner, BAG_LID_COLLIDER_GROUP, RADIUS,
    },
    conveyor_belt,
    conveyor_belt::BeltEmptyEvent,
    nominos::{NominoMarker, PiecePlaced, NOMINO_COLLIDER_GROUP},
};

pub struct BagReplacementPlugin;

impl Plugin for BagReplacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BagFilled>();
        app.add_event::<RemoveFilledBag>();
        app.add_event::<ReplaceFilledBag>();

        app.add_system_to_stage(CoreStage::PostUpdate, detect_filled_bags);
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            replace_full_bags.after(detect_filled_bags),
        );
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            remove_filled_bags
                .after(replace_full_bags)
                .after(TransformPropagate),
        );
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            replace_filled_bags
                .after(replace_full_bags)
                .after(TransformPropagate),
        );
    }
}

#[derive(Deref)]
struct BagFilled(Entity);

#[derive(Deref)]
struct RemoveFilledBag(Entity);

#[derive(Deref)]
struct ReplaceFilledBag(Entity);

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

#[derive(Debug, Default)]
pub enum BagReplacementFsm {
    #[default]
    Ready,
    BeltEmpty,
}

fn replace_full_bags(
    mut replacement_fsm: Local<BagReplacementFsm>,
    mut filled_events: EventReader<BagFilled>,
    mut belt_empty_events: EventReader<BeltEmptyEvent>,
    mut remove_events: EventWriter<RemoveFilledBag>,
    mut replace_events: EventWriter<ReplaceFilledBag>,
    bags: Query<Entity, With<BagMarker>>,
) {
    let belt_empty = belt_empty_events.iter().count() > 0;

    match *replacement_fsm {
        BagReplacementFsm::Ready => {
            for filled_bag in filled_events.iter() {
                remove_events.send(RemoveFilledBag(**filled_bag));
                replace_events.send(ReplaceFilledBag(**filled_bag));
            }

            if belt_empty {
                *replacement_fsm = BagReplacementFsm::BeltEmpty;
            }
        }
        BagReplacementFsm::BeltEmpty => {
            // Consume pending events
            filled_events.iter().count();

            for bag in bags.iter() {
                remove_events.send(RemoveFilledBag(bag));
            }
            *replacement_fsm = default();
        }
    }
}

fn remove_filled_bags(
    mut commands: Commands,
    mut remove_events: EventReader<RemoveFilledBag>,
    game_speed: Res<GameSpeed>,
    bag_positions: Query<&Transform, With<BagMarker>>,
    bag_pieces: Query<&BagPieces, With<BagMarker>>,
    mut piece_positions: Query<
        (&GlobalTransform, &mut Transform),
        (With<NominoMarker>, Without<BagMarker>),
    >,
) {
    for removed_bag in remove_events.iter() {
        let current_bag_position = bag_positions.get(**removed_bag).unwrap();
        let exit_bag_position = {
            let mut p = *current_bag_position;
            p.translation.y = -(RADIUS + 0.5);
            p
        };

        commands.entity(**removed_bag).insert(animations::bag_exit(
            *current_bag_position,
            exit_bag_position,
            &game_speed,
        ));

        // TODO remove after https://github.com/dimforge/bevy_rapier/issues/172
        for piece in &**bag_pieces.get(**removed_bag).unwrap() {
            commands.entity(**removed_bag).add_child(*piece);

            let bag_global = current_bag_position.translation;
            let (piece_global, mut piece_local) = piece_positions.get_mut(*piece).unwrap();
            piece_local.translation = piece_global.translation - bag_global;
        }
    }
}

fn replace_filled_bags(
    mut commands: Commands,
    mut replace_events: EventReader<ReplaceFilledBag>,
    game_speed: Res<GameSpeed>,
    bag_positions: Query<&Transform, With<BagMarker>>,
    bag_container: Query<Entity, With<BagContainerMarker>>,
) {
    for replaced_bag in replace_events.iter() {
        let current_bag_position = bag_positions.get(**replaced_bag).unwrap();
        let new_bag_start = {
            let mut p = *current_bag_position;
            p.scale = Vec3::ZERO;
            p
        };

        commands
            .entity(bag_container.single())
            .with_children(|parent| {
                parent
                    .spawn_bag_into(new_bag_start)
                    .insert(animations::bag_enter(
                        new_bag_start,
                        *current_bag_position,
                        &game_speed,
                    ));
            });
    }
}
