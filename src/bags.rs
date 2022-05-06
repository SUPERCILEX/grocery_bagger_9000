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
    ) -> [(Transform, Entity); N];
}

impl<'w, 's, 'a> BagSpawner for ChildBuilder<'w, 's, 'a> {
    fn spawn_bag<const N: usize>(
        &mut self,
        color: Color,
        window: &DipsWindow,
    ) -> [(Transform, Entity); N] {
        compute_bag_coordinates::<N>(window).map(|mut transform| {
            let id = spawn_bag(self, color, transform);
            // Adjust bag coordinates such that the canvas is centered on the bottom left corner
            transform.translation -= Vec3::new(RADIUS, RADIUS, 0.);
            (transform, id)
        })
    }
}

fn compute_bag_coordinates<const N: usize>(window: &DipsWindow) -> [Transform; N] {
    assert!(N > 0);

    let mut positions = [default(); N];
    if N == 1 {
        positions[0] = Transform::from_xyz(
            BagCoord(window.width / 2.).snap_to_grid(),
            BagCoord((window.height - conveyor_belt::HEIGHT) / 2.).snap_to_grid(),
            0.,
        );
    } else {
        todo!("Figure out how to compute positions for multiple bags. Ideally doesn't need an if statement.")
    }
    positions
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
