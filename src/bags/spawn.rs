use bevy::prelude::*;
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

pub trait BagSpawner {
    fn spawn_bag<const N: usize>(&mut self, window: &DipsWindow) -> SmallVec<[Entity; 3]>;
}

impl<'w, 's, 'a> BagSpawner for ChildBuilder<'w, 's, 'a> {
    fn spawn_bag<const N: usize>(&mut self, window: &DipsWindow) -> SmallVec<[Entity; 3]> {
        let mut spawned_bags = SmallVec::new();
        for position in compute_bag_coordinates(window, N) {
            spawned_bags.push(spawn_bag(self, Transform::from_translation(position)))
        }
        spawned_bags
    }
}

fn spawn_bag(commands: &mut ChildBuilder, transform: Transform) -> Entity {
    let draw_mode = DrawMode::Outlined {
        fill_mode: FillMode {
            options: FillOptions::default().with_intersections(false),
            color: BAG_COLOR,
        },
        outline_mode: StrokeMode::new(Color::BLACK, 0.15),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(&*BAG_PATH, draw_mode, transform))
        .insert(BagMarker)
        .insert(BAG_MAIN_COLLIDER.clone())
        .insert(Sensor(true))
        .insert(BAG_COLLIDER_GROUP)
        .insert(RigidBody::Fixed)
        .insert(BagPieces(SmallVec::default()))
        .with_children(|parent| {
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
        })
        .id()
}
