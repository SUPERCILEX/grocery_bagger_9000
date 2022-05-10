use std::lazy::SyncLazy;

use bevy::{math::const_vec3, prelude::*};
use bevy_prototype_lyon::prelude::{
    tess::{math::Point, path::path::Builder},
    *,
};
use bevy_rapier3d::prelude::*;

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
pub const BAG_ORIGIN: Vec3 = const_vec3!([RADIUS, RADIUS, 0.]);
pub const BAG_SPACING: f32 = 2.;
pub const BAG_CAPACITY: usize = 36;

pub static BAG_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(4, 4);

    b.begin(Point::new(-RADIUS, RADIUS));
    b.line_to(Point::new(-RADIUS, -RADIUS));
    b.line_to(Point::new(RADIUS, -RADIUS));
    b.line_to(Point::new(RADIUS, RADIUS));
    b.end(false);

    Path(b.build())
});

pub static MAIN_BAG_COLLIDER: SyncLazy<Collider> =
    SyncLazy::new(|| Collider::cuboid(RADIUS, RADIUS, 0.));

pub static BOUNDARY_BAG_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
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

pub static LID_BAG_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![(
        Vec3::new(0., RADIUS + 0.5, 0.),
        Quat::IDENTITY,
        Collider::cuboid(RADIUS, 0.49, 0.),
    )])
});
