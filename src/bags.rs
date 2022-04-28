use bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        tess::{math::Point, path::path::Builder},
        *,
    },
};
use bevy_rapier3d::prelude::{Point as PhysPoint, *};

use crate::dpi::Dips;

pub const RADIUS: Dips = Dips(3.);

pub trait Bag {
    fn path(&self) -> &Path;

    fn vertices(&self) -> &[PhysPoint<Real>];
}

#[derive(Bundle)]
pub struct BagBundle {
    #[bundle]
    shape: ShapeBundle,
    #[bundle]
    collider: ColliderBundle,
}

impl BagBundle {
    pub fn new(bag: impl Bag, color: Color, transform: Transform) -> Self {
        let collider = ColliderBundle {
            shape: ColliderShape::convex_hull(bag.vertices()).unwrap().into(),
            position: (transform.translation, transform.rotation).into(),
            ..default()
        };

        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode {
                options: FillOptions::default().with_intersections(false),
                color,
            },
            outline_mode: StrokeMode::new(Color::BLACK, 0.15),
        };

        Self {
            shape: GeometryBuilder::build_as(bag.path(), draw_mode, transform),
            collider,
        }
    }
}

pub struct Level1Bag {
    path: Path,
    vertices: [PhysPoint<Real>; 10],
}

impl Default for Level1Bag {
    fn default() -> Self {
        let mut b = Builder::with_capacity(10, 11);

        b.begin(Point::new(0., 0.));
        b.line_to(Point::new(0., 6.));
        b.line_to(Point::new(2., 6.));
        b.line_to(Point::new(2., 4.));
        b.line_to(Point::new(3., 4.));
        b.line_to(Point::new(3., 5.));
        b.line_to(Point::new(5., 5.));
        b.line_to(Point::new(5., 6.));
        b.line_to(Point::new(6., 6.));
        b.line_to(Point::new(6., 0.));
        b.close();

        let path = Path(b.build());

        let vertices: [PhysPoint<Real>; 10] = [
            Vec3::new(0., 0., 0.).into(),
            Vec3::new(0., 6., 0.).into(),
            Vec3::new(2., 6., 0.).into(),
            Vec3::new(2., 4., 0.).into(),
            Vec3::new(3., 4., 0.).into(),
            Vec3::new(3., 5., 0.).into(),
            Vec3::new(5., 5., 0.).into(),
            Vec3::new(5., 6., 0.).into(),
            Vec3::new(6., 6., 0.).into(),
            Vec3::new(6., 0., 0.).into(),
        ];

        Self { path, vertices }
    }
}

impl Bag for Level1Bag {
    fn path(&self) -> &Path {
        &self.path
    }

    fn vertices(&self) -> &[PhysPoint<Real>] {
        &self.vertices
    }
}
