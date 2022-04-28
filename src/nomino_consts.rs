use std::{f32::consts::PI, lazy::SyncLazy};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    tess::{math::Point, path::path::Builder},
    *,
};
use bevy_rapier3d::prelude::*;

pub static ROTATION_90: SyncLazy<Quat> = SyncLazy::new(|| Quat::from_rotation_z(-PI / 2.));

pub static TETROMINO_STRAIGHT_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 4.));
    b.line_to(Point::new(1., 4.));
    b.line_to(Point::new(1., 0.));
    b.close();

    Path(b.build())
});

pub static TETROMINO_STRAIGHT_COLLIDER: SyncLazy<ColliderShape> = SyncLazy::new(|| {
    ColliderShape::compound(vec![(
        Vec3::new(0.5, 2., 0.).into(),
        ColliderShape::cuboid(0.5, 2., 0.),
    )])
});

pub static TETROMINO_SQUARE_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 2.));
    b.line_to(Point::new(2., 2.));
    b.line_to(Point::new(2., 0.));
    b.close();

    Path(b.build())
});

pub static TETROMINO_SQUARE_COLLIDER: SyncLazy<ColliderShape> = SyncLazy::new(|| {
    ColliderShape::compound(vec![(
        Vec3::new(1., 1., 0.).into(),
        ColliderShape::cuboid(1., 1., 0.),
    )])
});

pub static TETROMINO_T_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(8, 9);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 3.));
    b.line_to(Point::new(1., 3.));
    b.line_to(Point::new(1., 2.));
    b.line_to(Point::new(2., 2.));
    b.line_to(Point::new(2., 1.));
    b.line_to(Point::new(1., 1.));
    b.line_to(Point::new(1., 0.));
    b.close();

    Path(b.build())
});

pub static TETROMINO_T_COLLIDER: SyncLazy<ColliderShape> = SyncLazy::new(|| {
    ColliderShape::compound(vec![
        (
            Vec3::new(0.5, 1.5, 0.).into(),
            ColliderShape::cuboid(0.5, 1.5, 0.),
        ),
        (
            Vec3::new(1.5, 1.5, 0.).into(),
            ColliderShape::cuboid(0.5, 0.5, 0.),
        ),
    ])
});

pub static TETROMINO_L_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(6, 7);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 3.));
    b.line_to(Point::new(1., 3.));
    b.line_to(Point::new(1., 1.));
    b.line_to(Point::new(2., 1.));
    b.line_to(Point::new(2., 0.));
    b.close();

    Path(b.build())
});

pub static TETROMINO_L_COLLIDER: SyncLazy<ColliderShape> = SyncLazy::new(|| {
    ColliderShape::compound(vec![
        (
            Vec3::new(0.5, 1.5, 0.).into(),
            ColliderShape::cuboid(0.5, 1.5, 0.),
        ),
        (
            Vec3::new(1.5, 0.5, 0.).into(),
            ColliderShape::cuboid(0.5, 0.5, 0.),
        ),
    ])
});

pub static TETROMINO_SKEW_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(8, 9);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 2.));
    b.line_to(Point::new(1., 2.));
    b.line_to(Point::new(1., 3.));
    b.line_to(Point::new(2., 3.));
    b.line_to(Point::new(2., 1.));
    b.line_to(Point::new(1., 1.));
    b.line_to(Point::new(1., 0.));
    b.close();

    Path(b.build())
});

pub static TETROMINO_SKEW_COLLIDER: SyncLazy<ColliderShape> = SyncLazy::new(|| {
    let sub_bar = ColliderShape::cuboid(0.5, 1., 0.);
    ColliderShape::compound(vec![
        (Vec3::new(0.5, 1., 0.).into(), sub_bar.clone()),
        (Vec3::new(1.5, 2., 0.).into(), sub_bar),
    ])
});
