use bevy::{math::const_vec3, prelude::*};
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    animations,
    animations::GameSpeed,
    bags::{
        spawn::{BagContainerMarker, BagLidMarker, BagMarker},
        BagSize, BagSpawner, BAG_LID_COLLIDER_GROUP, BAG_SIZE_LARGE,
    },
    conveyor_belt::BeltEmptyEvent,
    nominos::{NominoColor, NominoMarker, PiecePlaced, PieceSystems, NOMINO_COLLIDER_GROUP},
};

pub struct BagReplacementPlugin;

impl Plugin for BagReplacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BagFilled>();
        app.add_event::<BagChanged>();
        app.add_event::<RemoveFilledBag>();
        app.add_event::<ReplaceFilledBag>();

        app.add_system(
            bag_change_detection
                .label(BagChangeDetectionSystems)
                .after(PieceSystems),
        );
        app.add_system(
            detect_overflowing_bags
                .label(BagReplacementDetectionSystems)
                .after(PieceSystems),
        );
        app.add_system(
            detect_filled_bags
                .label(BagReplacementDetectionSystems)
                .after(BagChangeDetectionSystems),
        );
        app.add_system(
            replace_full_bags
                .label(BagReplacementSystems)
                .after(BagReplacementDetectionSystems),
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
pub struct BagChangeDetectionSystems;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct BagReplacementDetectionSystems;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct BagReplacementSystems;

#[derive(Deref)]
pub struct BagFilled(Entity);

pub struct BagChanged {
    pub bag: Entity,
    pub blocks: SmallVec<
        [SmallVec<[Option<NominoColor>; BAG_SIZE_LARGE.width() as usize]>;
            BAG_SIZE_LARGE.height() as usize],
    >,
}

#[derive(Deref)]
struct RemoveFilledBag(Entity);

#[derive(Deref)]
struct ReplaceFilledBag(Entity);

#[derive(Component)]
pub struct Exiting;

fn bag_change_detection(
    mut piece_placements: EventReader<PiecePlaced>,
    mut bag_changes: EventWriter<BagChanged>,
    bags: Query<(&GlobalTransform, &BagSize), With<BagMarker>>,
    color_wrapper: Query<&NominoColor, With<NominoMarker>>,
    rapier_context: Res<RapierContext>,
) {
    for PiecePlaced { bag, .. } in piece_placements.iter() {
        let (bag_coords, bag_size) = bags.get(*bag).unwrap();

        let width = bag_size.width() as usize;
        let height = bag_size.height() as usize;
        let block_origin = bag_coords.translation - bag_size.origin() + const_vec3!([0.5, 0.5, 0.]);

        let mut blocks = SmallVec::with_capacity(height);
        for row_num in 0..height {
            let mut row = SmallVec::with_capacity(width);
            for col in 0..width {
                let mut color = None;
                rapier_context.intersections_with_point(
                    block_origin
                        + Vec3::new(
                            f32::from(u8::try_from(col).unwrap()),
                            f32::from(u8::try_from(row_num).unwrap()),
                            0.,
                        ),
                    NOMINO_COLLIDER_GROUP.into(),
                    None,
                    |color_id| {
                        color = Some(*color_wrapper.get(color_id).unwrap());
                        false
                    },
                );

                row.push(color);
            }
            blocks.push(row);
        }

        bag_changes.send(BagChanged { bag: *bag, blocks });
    }
}

fn detect_overflowing_bags(
    mut piece_placements: EventReader<PiecePlaced>,
    mut filled_events: EventWriter<BagFilled>,
    rapier_context: Res<RapierContext>,
    piece_colliders: Query<(&GlobalTransform, &Collider), With<NominoMarker>>,
    lid_collider_bag: Query<&Parent, With<BagLidMarker>>,
) {
    for PiecePlaced { piece, bag } in piece_placements.iter() {
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
        }
    }
}

fn detect_filled_bags(
    mut bag_changes: EventReader<BagChanged>,
    mut filled_events: EventWriter<BagFilled>,
) {
    for BagChanged { bag, blocks } in bag_changes.iter() {
        let top_row_full = blocks.iter().last().unwrap().iter().all(Option::is_some);
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
    bags: Query<(&Transform, &GlobalTransform, &BagSize), With<BagMarker>>,
) {
    for removed_bag in remove_events.iter() {
        let (local, global, bag_size) = bags.get(**removed_bag).unwrap();
        let exit_bag_position = {
            let mut p = *local;
            p.translation.y = -(global.translation.y + bag_size.half_height());
            p
        };

        commands
            .entity(**removed_bag)
            .insert(animations::bag_exit(*local, exit_bag_position, &game_speed))
            .insert(Exiting);
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
        commands
            .entity(bag_container.single())
            .with_children(|parent| {
                parent.spawn_replacement_bag(&game_speed, *current_bag_position, *bag_size);
            });
    }
}
