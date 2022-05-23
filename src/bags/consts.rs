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
pub const BAG_LID_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b1000,
    filters: 0b1000,
};
pub const BAG_WALLS_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b100,
    filters: 0b100,
};
pub const BAG_FLOOR_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b10000,
    filters: 0b10000,
};
pub const BAG_BOUNDARY_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: BAG_WALLS_COLLIDER_GROUP.memberships | BAG_FLOOR_COLLIDER_GROUP.memberships,
    filters: BAG_WALLS_COLLIDER_GROUP.filters | BAG_FLOOR_COLLIDER_GROUP.filters,
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

pub static BAG_MAIN_COLLIDER: SyncLazy<Collider> =
    SyncLazy::new(|| Collider::cuboid(RADIUS, RADIUS, 0.));

pub static BAG_LID_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![(
        const_vec3!([0., RADIUS + 0.5, 0.]),
        Quat::IDENTITY,
        Collider::cuboid(RADIUS, 0.49, 0.),
    )])
});

pub static BAG_WALLS_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![
        (
            const_vec3!([-RADIUS, 0., 0.]),
            Quat::IDENTITY,
            Collider::cuboid(0.009, RADIUS, 0.),
        ),
        (
            const_vec3!([RADIUS, 0., 0.]),
            Quat::IDENTITY,
            Collider::cuboid(0.009, RADIUS, 0.),
        ),
    ])
});

pub static BAG_FLOOR_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![(
        const_vec3!([0., -RADIUS, 0.]),
        Quat::IDENTITY,
        Collider::cuboid(RADIUS, 0.009, 0.),
    )])
});

pub const BAG_COLOR: Color = Color::rgb(0xC3 as f32 / 255., 0xA9 as f32 / 255., 0x88 as f32 / 255.);
pub const BAG_OUTLINE_COLOR: Color =
    Color::rgb(0x64 as f32 / 255., 0x56 as f32 / 255., 0x46 as f32 / 255.);
