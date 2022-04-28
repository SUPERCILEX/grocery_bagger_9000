use std::{f32::consts::PI, lazy::SyncLazy};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    tess::{math::Point, path::path::Builder},
    *,
};
use bevy_rapier3d::prelude::{Point as PhysPoint, *};

pub static ROTATION_90: SyncLazy<Quat> = SyncLazy::new(|| Quat::from_rotation_z(-PI / 2.));

pub static TETROMINO_STRAIGHT_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 1.));
    b.line_to(Point::new(4., 1.));
    b.line_to(Point::new(4., 0.));
    b.close();

    Path(b.build())
});

pub static TETROMINO_STRAIGHT_VERTICES: SyncLazy<[PhysPoint<Real>; 4]> = SyncLazy::new(|| {
    [
        Vec3::new(0., 0., 0.).into(),
        Vec3::new(0., 1., 0.).into(),
        Vec3::new(4., 1., 0.).into(),
        Vec3::new(4., 0., 0.).into(),
    ]
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

pub static TETROMINO_SQUARE_VERTICES: SyncLazy<[PhysPoint<Real>; 4]> = SyncLazy::new(|| {
    [
        Vec3::new(0., 0., 0.).into(),
        Vec3::new(0., 2., 0.).into(),
        Vec3::new(2., 2., 0.).into(),
        Vec3::new(2., 0., 0.).into(),
    ]
});

pub static TETROMINO_T_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(8, 9);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 1.));
    b.line_to(Point::new(1., 1.));
    b.line_to(Point::new(1., 2.));
    b.line_to(Point::new(2., 2.));
    b.line_to(Point::new(2., 1.));
    b.line_to(Point::new(3., 1.));
    b.line_to(Point::new(3., 0.));
    b.close();

    Path(b.build())
});

pub static TETROMINO_T_VERTICES: SyncLazy<[PhysPoint<Real>; 8]> = SyncLazy::new(|| {
    [
        Vec3::new(0., 0., 0.).into(),
        Vec3::new(0., 1., 0.).into(),
        Vec3::new(1., 1., 0.).into(),
        Vec3::new(1., 2., 0.).into(),
        Vec3::new(2., 2., 0.).into(),
        Vec3::new(2., 1., 0.).into(),
        Vec3::new(3., 1., 0.).into(),
        Vec3::new(3., 0., 0.).into(),
    ]
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

pub static TETROMINO_L_VERTICES: SyncLazy<[PhysPoint<Real>; 6]> = SyncLazy::new(|| {
    [
        Vec3::new(0., 0., 0.).into(),
        Vec3::new(0., 3., 0.).into(),
        Vec3::new(1., 3., 0.).into(),
        Vec3::new(1., 1., 0.).into(),
        Vec3::new(2., 1., 0.).into(),
        Vec3::new(2., 0., 0.).into(),
    ]
});

pub static TETROMINO_SKEW_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(8, 9);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 1.));
    b.line_to(Point::new(1., 1.));
    b.line_to(Point::new(1., 2.));
    b.line_to(Point::new(3., 2.));
    b.line_to(Point::new(3., 1.));
    b.line_to(Point::new(2., 1.));
    b.line_to(Point::new(2., 0.));
    b.close();

    Path(b.build())
});

pub static TETROMINO_SKEW_VERTICES: SyncLazy<[PhysPoint<Real>; 8]> = SyncLazy::new(|| {
    [
        Vec3::new(0., 0., 0.).into(),
        Vec3::new(0., 1., 0.).into(),
        Vec3::new(1., 1., 0.).into(),
        Vec3::new(1., 2., 0.).into(),
        Vec3::new(3., 2., 0.).into(),
        Vec3::new(3., 1., 0.).into(),
        Vec3::new(2., 1., 0.).into(),
        Vec3::new(2., 0., 0.).into(),
    ]
});
