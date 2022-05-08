use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{FillMode, *};
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    bags::{bag_replacement::BagPieces, consts::*, positioning::compute_bag_coordinates},
    window_management::DipsWindow,
};

pub trait BagSpawner {
    fn spawn_bag<const N: usize>(
        &mut self,
        color: Color,
        window: &DipsWindow,
    ) -> SmallVec<[(Transform, Entity); 3]>;
}

impl<'w, 's, 'a> BagSpawner for ChildBuilder<'w, 's, 'a> {
    fn spawn_bag<const N: usize>(
        &mut self,
        color: Color,
        window: &DipsWindow,
    ) -> SmallVec<[(Transform, Entity); 3]> {
        let mut bag_positions = compute_bag_coordinates(window, N);
        let mut spawned_bags = SmallVec::new();
        for position in &mut bag_positions {
            let mut position = Transform::from_translation(*position);
            let id = spawn_bag(self, color, position);

            // Adjust bag coordinates such that the canvas is centered on the bottom left
            // corner
            position.translation -= Vec3::new(RADIUS, RADIUS, 0.);

            spawned_bags.push((position, id))
        }
        spawned_bags
    }
}

fn spawn_bag(commands: &mut ChildBuilder, color: Color, transform: Transform) -> Entity {
    let draw_mode = DrawMode::Outlined {
        fill_mode: FillMode {
            options: FillOptions::default().with_intersections(false),
            color,
        },
        outline_mode: StrokeMode::new(Color::BLACK, 0.15),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(&*BAG_PATH, draw_mode, transform))
        .insert(MAIN_BAG_COLLIDER.clone())
        .insert(Sensor(true))
        .insert(BAG_COLLIDER_GROUP)
        .insert(RigidBody::Fixed)
        .insert(BagPieces(SmallVec::default()))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(BOUNDARY_BAG_COLLIDER.clone())
                .insert(Sensor(true))
                .insert(BAG_BOUNDARY_COLLIDER_GROUP);

            parent
                .spawn()
                .insert(LID_BAG_COLLIDER.clone())
                .insert(Sensor(true))
                .insert(BAG_LID_COLLIDER_GROUP);
        })
        .id()
}
