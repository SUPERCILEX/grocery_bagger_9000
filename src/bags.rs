use std::lazy::SyncLazy;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    tess::{math::Point, path::path::Builder},
    FillMode, *,
};
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{conveyor_belt, window_management::DipsWindow};

pub const BAG_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b10,
    filters: 0b10,
};
pub const BAG_BOUNDARY_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b100,
    filters: 0b100,
};
pub const BAG_LID_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b1000,
    filters: 0b1000,
};

pub const RADIUS: f32 = 3.;
pub const BAG_OFFSET: f32 = 2.5;
const BAG_SPACING: f32 = 2.;

static BAG_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(4, 4);

    b.begin(Point::new(-RADIUS, RADIUS));
    b.line_to(Point::new(-RADIUS, -RADIUS));
    b.line_to(Point::new(RADIUS, -RADIUS));
    b.line_to(Point::new(RADIUS, RADIUS));
    b.end(false);

    Path(b.build())
});

static MAIN_BAG_COLLIDER: SyncLazy<Collider> =
    SyncLazy::new(|| Collider::cuboid(RADIUS, RADIUS, 0.));

static BOUNDARY_BAG_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![
        (
            Vec3::new(-RADIUS, 0., 0.),
            Quat::IDENTITY,
            Collider::cuboid(0.009, RADIUS, 0.),
        ),
        (
            Vec3::new(0., -RADIUS, 0.),
            Quat::IDENTITY,
            Collider::cuboid(RADIUS, 0.009, 0.),
        ),
        (
            Vec3::new(RADIUS, 0., 0.),
            Quat::IDENTITY,
            Collider::cuboid(0.009, RADIUS, 0.),
        ),
    ])
});

static LID_BAG_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![(
        Vec3::new(0., RADIUS + 0.5, 0.),
        Quat::IDENTITY,
        Collider::cuboid(RADIUS, 0.49, 0.),
    )])
});

#[derive(Component, Deref, DerefMut)]
pub struct BagPieces(pub SmallVec<[Entity; conveyor_belt::MAX_NUM_PIECES]>);

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

            // Adjust bag coordinates such that the canvas is centered on the bottom left corner
            position.translation -= Vec3::new(RADIUS, RADIUS, 0.);

            spawned_bags.push((position, id))
        }
        spawned_bags
    }
}

pub fn compute_bag_coordinates(window: &DipsWindow, num_bags: usize) -> SmallVec<[Vec3; 3]> {
    debug_assert!(num_bags != 0);

    let space_needed = 2. * RADIUS * num_bags as f32 + (num_bags - 1) as f32 * BAG_SPACING;
    let starting_position = (window.width - space_needed) / 2. + RADIUS;
    debug_assert!(starting_position >= 0. && starting_position <= window.width);

    let mut bags = SmallVec::new();
    for bag in 0..num_bags {
        bags.push(Vec3::new(
            BagCoord(starting_position + (2. * RADIUS * bag as f32 + bag as f32 * BAG_SPACING))
                .snap_to_grid(),
            BagCoord((window.height - conveyor_belt::HEIGHT) / 2.).snap_to_grid(),
            0.,
        ))
    }
    bags
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

pub trait BagSnapper<T> {
    fn snap_to_grid(&self) -> T;
}

#[derive(Deref)]
pub struct BagCoord(pub f32);

impl BagSnapper<f32> for BagCoord {
    fn snap_to_grid(&self) -> f32 {
        self.round() + 0.5
    }
}
