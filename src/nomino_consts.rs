use std::{f32::consts::PI, lazy::SyncLazy};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    tess::{math::Point, path::path::Builder},
    *,
};
use bevy_rapier3d::prelude::*;

pub static DEG_90: SyncLazy<Quat> = SyncLazy::new(|| Quat::from_rotation_z(-PI / 2.));
pub static DEG_180: SyncLazy<Quat> = SyncLazy::new(|| Quat::from_rotation_z(-PI));

pub static TETROMINO_STRAIGHT_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(-0.5, -2.5));
    b.line_to(Point::new(-0.5, 1.5));
    b.line_to(Point::new(0.5, 1.5));
    b.line_to(Point::new(0.5, -2.5));
    b.close();

    Path(b.build())
});

pub static TETROMINO_STRAIGHT_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![(
        Vec3::new(0., -0.5, 0.),
        Quat::IDENTITY,
        Collider::cuboid(0.49, 1.99, 0.),
    )])
});

pub static TETROMINO_SQUARE_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(-0.5, -0.5));
    b.line_to(Point::new(-0.5, 1.5));
    b.line_to(Point::new(1.5, 1.5));
    b.line_to(Point::new(1.5, -0.5));
    b.close();

    Path(b.build())
});

pub static TETROMINO_SQUARE_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![(
        Vec3::new(0.5, 0.5, 0.),
        Quat::IDENTITY,
        Collider::cuboid(0.99, 0.99, 0.),
    )])
});

pub static TETROMINO_T_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(8, 9);

    b.begin(Point::new(-0.5, -1.5));
    b.line_to(Point::new(-0.5, 1.5));
    b.line_to(Point::new(0.5, 1.5));
    b.line_to(Point::new(0.5, 0.5));
    b.line_to(Point::new(1.5, 0.5));
    b.line_to(Point::new(1.5, -0.5));
    b.line_to(Point::new(0.5, -0.5));
    b.line_to(Point::new(0.5, -1.5));
    b.close();

    Path(b.build())
});

pub static TETROMINO_T_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![
        (Vec3::ZERO, Quat::IDENTITY, Collider::cuboid(0.49, 1.49, 0.)),
        (
            Vec3::new(0.99, 0., 0.),
            Quat::IDENTITY,
            Collider::cuboid(0.5, 0.49, 0.),
        ),
    ])
});

pub static TETROMINO_L_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(6, 7);

    b.begin(Point::new(-0.5, -1.5));
    b.line_to(Point::new(-0.5, 1.5));
    b.line_to(Point::new(0.5, 1.5));
    b.line_to(Point::new(0.5, -0.5));
    b.line_to(Point::new(1.5, -0.5));
    b.line_to(Point::new(1.5, -1.5));
    b.close();

    Path(b.build())
});

pub static TETROMINO_L_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    Collider::compound(vec![
        (Vec3::ZERO, Quat::IDENTITY, Collider::cuboid(0.49, 1.49, 0.)),
        (
            Vec3::new(0.99, -1., 0.),
            Quat::IDENTITY,
            Collider::cuboid(0.5, 0.49, 0.),
        ),
    ])
});

pub static TETROMINO_SKEW_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(8, 9);

    b.begin(Point::new(-0.5, -1.5));
    b.line_to(Point::new(-0.5, 0.5));
    b.line_to(Point::new(0.5, 0.5));
    b.line_to(Point::new(0.5, 1.5));
    b.line_to(Point::new(1.5, 1.5));
    b.line_to(Point::new(1.5, -0.5));
    b.line_to(Point::new(0.5, -0.5));
    b.line_to(Point::new(0.5, -1.5));
    b.close();

    Path(b.build())
});

pub static TETROMINO_SKEW_COLLIDER: SyncLazy<Collider> = SyncLazy::new(|| {
    let sub_bar = Collider::cuboid(0.49, 0.99, 0.);
    Collider::compound(vec![
        (Vec3::new(0., -0.5, 0.), Quat::IDENTITY, sub_bar.clone()),
        (Vec3::new(1., 0.5, 0.), Quat::IDENTITY, sub_bar),
        (
            Vec3::new(0.5, 0., 0.),
            Quat::IDENTITY,
            Collider::cuboid(0.1, 0.49, 0.),
        ),
    ])
});
