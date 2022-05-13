use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::{FillMode, *};
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    bags::{bag_replacement::BagPieces, consts::*, positioning::compute_bag_coordinates},
    window_management::DipsWindow,
};

const BAG_COLOR: Color = Color::rgb(0xE6 as f32 / 255., 0xE6 as f32 / 255., 0xE6 as f32 / 255.);

#[derive(Component)]
pub struct BagMarker;

#[derive(Component)]
pub struct BagLidMarker;

#[derive(Component)]
pub struct BagWallsMarker;

#[derive(Component)]
pub struct BagFloorMarker;

pub trait BagSpawner<'w, 's> {
    fn spawn_bag<const N: usize>(&mut self, window: &DipsWindow) -> SmallVec<[Entity; 3]>;

    fn spawn_bag_into(&mut self, transform: Transform) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's, 'a> BagSpawner<'w, 's> for ChildBuilder<'w, 's, 'a> {
    fn spawn_bag<const N: usize>(&mut self, window: &DipsWindow) -> SmallVec<[Entity; 3]> {
        let mut spawned_bags = SmallVec::new();
        for position in compute_bag_coordinates(window, N) {
            spawned_bags.push(spawn_bag(self, Transform::from_translation(position)).id())
        }
        spawned_bags
    }

    fn spawn_bag_into(&mut self, transform: Transform) -> EntityCommands<'w, 's, '_> {
        spawn_bag(self, transform)
    }
}

fn spawn_bag<'w, 's, 'a>(
    commands: &'a mut ChildBuilder<'w, 's, '_>,
    transform: Transform,
) -> EntityCommands<'w, 's, 'a> {
    let draw_mode = DrawMode::Outlined {
        fill_mode: FillMode {
            options: FillOptions::default().with_intersections(false),
            color: BAG_COLOR,
        },
        outline_mode: StrokeMode::new(Color::BLACK, 0.15),
    };

    let mut commands =
        commands.spawn_bundle(GeometryBuilder::build_as(&*BAG_PATH, draw_mode, transform));
    commands.insert(BagMarker);
    commands.insert(BAG_MAIN_COLLIDER.clone());
    commands.insert(Sensor(true));
    commands.insert(BAG_COLLIDER_GROUP);
    commands.insert(RigidBody::Fixed);
    commands.insert(BagPieces(SmallVec::default()));
    commands.with_children(|parent| {
        parent
            .spawn()
            .insert(BagLidMarker)
            .insert(BAG_LID_COLLIDER.clone())
            .insert(Sensor(true))
            .insert(BAG_LID_COLLIDER_GROUP);

        parent
            .spawn()
            .insert(BagWallsMarker)
            .insert(BAG_WALLS_COLLIDER.clone())
            .insert(Sensor(true))
            .insert(BAG_WALLS_COLLIDER_GROUP);

        parent
            .spawn()
            .insert(BagFloorMarker)
            .insert(BAG_FLOOR_COLLIDER.clone())
            .insert(Sensor(true))
            .insert(BAG_FLOOR_COLLIDER_GROUP);
    });
    commands
}
