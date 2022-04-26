use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    tess::{math::Point, path::path::Builder},
    *,
};
use static_init::dynamic;

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

pub const TETROMINO_STRAIGHT_BOUNDING_BOXES: &[Rect<f32>] = &[Rect {
    bottom: 0.,
    left: 0.,
    top: 1.,
    right: 4.,
}];

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

pub const TETROMINO_SQUARE_BOUNDING_BOXES: &[Rect<f32>] = &[Rect {
    bottom: 0.,
    left: 0.,
    top: 2.,
    right: 2.,
}];

#[dynamic]
pub static TETROMINO_T_PATH: Path = {
    let mut b = Builder::with_capacity(7, 8);

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

pub const TETROMINO_T_BOUNDING_BOXES: &[Rect<f32>] = &[
    Rect {
        bottom: 0.,
        left: 0.,
        top: 1.,
        right: 3.,
    },
    Rect {
        bottom: 1.,
        left: 1.,
        top: 2.,
        right: 2.,
    },
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

pub const TETROMINO_L_BOUNDING_BOXES: &[Rect<f32>] = &[
    Rect {
        bottom: 0.,
        left: 0.,
        top: 3.,
        right: 1.,
    },
    Rect {
        bottom: 0.,
        left: 1.,
        top: 1.,
        right: 2.,
    },
];

#[dynamic]
pub static TETROMINO_SKEW_PATH: Path = {
    let mut b = Builder::with_capacity(7, 8);

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

pub const TETROMINO_SKEW_BOUNDING_BOXES: &[Rect<f32>] = &[
    Rect {
        bottom: 0.,
        left: 0.,
        top: 1.,
        right: 2.,
    },
    Rect {
        bottom: 1.,
        left: 1.,
        top: 2.,
        right: 3.,
    },
];
