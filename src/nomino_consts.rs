use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    tess::{math::Point, path::path::Builder},
    *,
};
use bevy_rapier3d::prelude::{Point as PhysPoint, *};
use static_init::dynamic;

#[dynamic]
pub static ROTATION_90: Quat = Quat::from_rotation_z(-PI / 2.);

#[dynamic]
pub static TETROMINO_STRAIGHT_PATH: Path = {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 1.));
    b.line_to(Point::new(4., 1.));
    b.line_to(Point::new(4., 0.));
    b.close();

    Path(b.build())
};

#[dynamic]
pub static TETROMINO_STRAIGHT_VERTICES: [PhysPoint<Real>; 4] = [
    Vec3::new(0., 0., 0.).into(),
    Vec3::new(0., 1., 0.).into(),
    Vec3::new(4., 1., 0.).into(),
    Vec3::new(4., 0., 0.).into(),
];

#[dynamic]
pub static TETROMINO_SQUARE_PATH: Path = {
    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 2.));
    b.line_to(Point::new(2., 2.));
    b.line_to(Point::new(2., 0.));
    b.close();

    Path(b.build())
};

#[dynamic]
pub static TETROMINO_SQUARE_VERTICES: [PhysPoint<Real>; 4] = [
    Vec3::new(0., 0., 0.).into(),
    Vec3::new(0., 2., 0.).into(),
    Vec3::new(2., 2., 0.).into(),
    Vec3::new(2., 0., 0.).into(),
];

#[dynamic]
pub static TETROMINO_T_PATH: Path = {
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
};

#[dynamic]
pub static TETROMINO_T_VERTICES: [PhysPoint<Real>; 8] = [
    Vec3::new(0., 0., 0.).into(),
    Vec3::new(0., 1., 0.).into(),
    Vec3::new(1., 1., 0.).into(),
    Vec3::new(1., 2., 0.).into(),
    Vec3::new(2., 2., 0.).into(),
    Vec3::new(2., 1., 0.).into(),
    Vec3::new(3., 1., 0.).into(),
    Vec3::new(3., 0., 0.).into(),
];

#[dynamic]
pub static TETROMINO_L_PATH: Path = {
    let mut b = Builder::with_capacity(6, 7);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(0., 3.));
    b.line_to(Point::new(1., 3.));
    b.line_to(Point::new(1., 1.));
    b.line_to(Point::new(2., 1.));
    b.line_to(Point::new(2., 0.));
    b.close();

    Path(b.build())
};

#[dynamic]
pub static TETROMINO_L_VERTICES: [PhysPoint<Real>; 6] = [
    Vec3::new(0., 0., 0.).into(),
    Vec3::new(0., 3., 0.).into(),
    Vec3::new(1., 3., 0.).into(),
    Vec3::new(1., 1., 0.).into(),
    Vec3::new(2., 1., 0.).into(),
    Vec3::new(2., 0., 0.).into(),
];

#[dynamic]
pub static TETROMINO_SKEW_PATH: Path = {
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
};

#[dynamic]
pub static TETROMINO_SKEW_VERTICES: [PhysPoint<Real>; 8] = [
    Vec3::new(0., 0., 0.).into(),
    Vec3::new(0., 1., 0.).into(),
    Vec3::new(1., 1., 0.).into(),
    Vec3::new(1., 2., 0.).into(),
    Vec3::new(3., 2., 0.).into(),
    Vec3::new(3., 1., 0.).into(),
    Vec3::new(2., 1., 0.).into(),
    Vec3::new(2., 0., 0.).into(),
];
