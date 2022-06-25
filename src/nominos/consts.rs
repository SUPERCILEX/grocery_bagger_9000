use std::{f32::consts::PI, sync::LazyLock};

use bevy::{math::const_vec3, prelude::*};
use bevy_prototype_lyon::prelude::{
    tess::{math::Point, path::path::Builder},
    *,
};
use bevy_rapier3d::prelude::*;

pub static DEG_90: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_z(-PI / 2.));
pub static DEG_180: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_z(PI));
pub static DEG_MIRRORED: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_y(PI));

pub static TROMINO_STRAIGHT_PATH: LazyLock<Path> = LazyLock::new(|| {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(-0.5, -1.5));
    b.line_to(Point::new(-0.5, 1.5));
    b.line_to(Point::new(0.5, 1.5));
    b.line_to(Point::new(0.5, -1.5));
    b.close();

    Path(b.build())
});

pub static TROMINO_STRAIGHT_COLLIDER: LazyLock<Collider> =
    LazyLock::new(|| Collider::cuboid(0.49, 1.49, 0.1));

pub static TROMINO_L_PATH: LazyLock<Path> = LazyLock::new(|| {
    let mut b = Builder::with_capacity(6, 7);

    b.begin(Point::new(-0.5, -0.5));
    b.line_to(Point::new(-0.5, 1.5));
    b.line_to(Point::new(0.5, 1.5));
    b.line_to(Point::new(0.5, 0.5));
    b.line_to(Point::new(1.5, 0.5));
    b.line_to(Point::new(1.5, -0.5));
    b.close();

    Path(b.build())
});

pub static TROMINO_L_COLLIDER: LazyLock<Collider> = LazyLock::new(|| {
    Collider::compound(vec![
        (
            const_vec3!([0., 0.5, 0.]),
            Quat::IDENTITY,
            Collider::cuboid(0.49, 0.99, 0.1),
        ),
        (
            const_vec3!([0.99, 0., 0.]),
            Quat::IDENTITY,
            Collider::cuboid(0.5, 0.49, 0.1),
        ),
    ])
});

pub static TETROMINO_STRAIGHT_PATH: LazyLock<Path> = LazyLock::new(|| {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(-0.5, -2.5));
    b.line_to(Point::new(-0.5, 1.5));
    b.line_to(Point::new(0.5, 1.5));
    b.line_to(Point::new(0.5, -2.5));
    b.close();

    Path(b.build())
});

pub static TETROMINO_STRAIGHT_COLLIDER: LazyLock<Collider> = LazyLock::new(|| {
    Collider::compound(vec![(
        const_vec3!([0., -0.5, 0.]),
        Quat::IDENTITY,
        Collider::cuboid(0.49, 1.99, 0.1),
    )])
});

pub static TETROMINO_SQUARE_PATH: LazyLock<Path> = LazyLock::new(|| {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(-0.5, -0.5));
    b.line_to(Point::new(-0.5, 1.5));
    b.line_to(Point::new(1.5, 1.5));
    b.line_to(Point::new(1.5, -0.5));
    b.close();

    Path(b.build())
});

pub static TETROMINO_SQUARE_COLLIDER: LazyLock<Collider> = LazyLock::new(|| {
    Collider::compound(vec![(
        const_vec3!([0.5, 0.5, 0.]),
        Quat::IDENTITY,
        Collider::cuboid(0.99, 0.99, 0.1),
    )])
});

pub static TETROMINO_T_PATH: LazyLock<Path> = LazyLock::new(|| {
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

pub static TETROMINO_T_COLLIDER: LazyLock<Collider> = LazyLock::new(|| {
    Collider::compound(vec![
        (
            Vec3::ZERO,
            Quat::IDENTITY,
            Collider::cuboid(0.49, 1.49, 0.1),
        ),
        (
            const_vec3!([0.99, 0., 0.]),
            Quat::IDENTITY,
            Collider::cuboid(0.5, 0.49, 0.1),
        ),
    ])
});

pub static TETROMINO_L_PATH: LazyLock<Path> = LazyLock::new(|| {
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

pub static TETROMINO_L_COLLIDER: LazyLock<Collider> = LazyLock::new(|| {
    Collider::compound(vec![
        (
            Vec3::ZERO,
            Quat::IDENTITY,
            Collider::cuboid(0.49, 1.49, 0.1),
        ),
        (
            const_vec3!([0.99, -1., 0.]),
            Quat::IDENTITY,
            Collider::cuboid(0.5, 0.49, 0.1),
        ),
    ])
});

pub static TETROMINO_SKEW_PATH: LazyLock<Path> = LazyLock::new(|| {
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

pub static TETROMINO_SKEW_COLLIDER: LazyLock<Collider> = LazyLock::new(|| {
    let sub_bar = Collider::cuboid(0.49, 0.99, 0.1);
    Collider::compound(vec![
        (const_vec3!([0., -0.5, 0.]), Quat::IDENTITY, sub_bar.clone()),
        (const_vec3!([1., 0.5, 0.]), Quat::IDENTITY, sub_bar),
        (
            const_vec3!([0.5, 0., 0.]),
            Quat::IDENTITY,
            Collider::cuboid(0.1, 0.49, 0.1),
        ),
    ])
});
