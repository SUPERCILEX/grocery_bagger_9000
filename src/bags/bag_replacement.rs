use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    animations,
    animations::GameSpeed,
    bags::{
        spawn::{BagContainerMarker, BagLidMarker, BagMarker},
        BagSize, BagSpawner, BAG_LID_COLLIDER_GROUP,
    },
    conveyor_belt,
    conveyor_belt::BeltEmptyEvent,
    nominos::{NominoMarker, PiecePlaced, PieceSystems, NOMINO_COLLIDER_GROUP},
};

pub struct BagReplacementPlugin;

impl Plugin for BagReplacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BagFilled>();
        app.add_event::<RemoveFilledBag>();
        app.add_event::<ReplaceFilledBag>();

        app.add_system(
            detect_filled_bags
                .label(BagSetupSystems)
                .after(PieceSystems),
        );
        app.add_system(
            replace_full_bags
                .label(BagReplacementSystems)
                .after(detect_filled_bags),
        );
        app.add_system(
            remove_filled_bags
                .label(BagReplacementSystems)
                .after(replace_full_bags),
        );
        app.add_system(
            replace_filled_bags
                .label(BagReplacementSystems)
                .after(replace_full_bags),
        );
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct BagSetupSystems;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct BagReplacementSystems;

#[derive(Deref)]
pub struct BagFilled(Entity);

#[derive(Deref)]
struct RemoveFilledBag(Entity);

#[derive(Deref)]
struct ReplaceFilledBag(Entity);

#[derive(Component, Deref, DerefMut)]
pub struct BagPieces(pub SmallVec<[Entity; conveyor_belt::MAX_NUM_PIECES as usize]>);

fn detect_filled_bags(
    mut piece_placements: EventReader<PiecePlaced>,
    mut bags: Query<(&GlobalTransform, &mut BagPieces, &BagSize), With<BagMarker>>,
    mut filled_events: EventWriter<BagFilled>,
    rapier_context: Res<RapierContext>,
    piece_colliders: Query<(&GlobalTransform, &Collider), With<NominoMarker>>,
    lid_collider_bag: Query<&Parent, With<BagLidMarker>>,
) {
    for PiecePlaced { piece, bag } in piece_placements.iter() {
        let (bag_coords, mut bag_pieces, bag_size) = bags.get_mut(*bag).unwrap();
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
        bag_coords.translation += Vec3::new(
            0.5 - bag_size.half_width(),
            bag_size.half_height() - 0.5,
            0.,
        );

        let mut top_row_full = true;
        for i in 0..bag_size.width() {
            let mut intersection = false;
            rapier_context.intersections_with_point(
                bag_coords.translation + Vec3::new(f32::from(i), 0., 0.),
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
    bags: Query<(&Transform, &BagSize), With<BagMarker>>,
    bag_pieces: Query<&BagPieces, With<BagMarker>>,
    mut piece_positions: Query<
        (&GlobalTransform, &mut Transform),
        (With<NominoMarker>, Without<BagMarker>),
    >,
) {
    for removed_bag in remove_events.iter() {
        let (current_bag_position, bag_size) = bags.get(**removed_bag).unwrap();
        let exit_bag_position = {
            let mut p = *current_bag_position;
            p.translation.y = -(bag_size.half_height() + 0.5);
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
    bags: Query<(&Transform, &BagSize), With<BagMarker>>,
    bag_container: Query<Entity, With<BagContainerMarker>>,
) {
    for replaced_bag in replace_events.iter() {
        let (current_bag_position, bag_size) = bags.get(**replaced_bag).unwrap();
        let new_bag_start = {
            let mut p = *current_bag_position;
            p.scale = Vec3::ZERO;
            p
        };

        commands
            .entity(bag_container.single())
            .with_children(|parent| {
                parent
                    .spawn_replacement_bag(new_bag_start, *bag_size)
                    .insert(animations::bag_enter(
                        new_bag_start,
                        *current_bag_position,
                        &game_speed,
                    ));
            });
    }
}
