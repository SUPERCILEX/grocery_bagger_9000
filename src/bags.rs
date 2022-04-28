use bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        tess::{math::Point, path::path::Builder},
        *,
    },
};
use bevy_rapier3d::prelude::*;

pub const RADIUS: f32 = 3.;

pub const BAG_COLLIDER_GROUP: InteractionGroups = InteractionGroups::new(1 << 1, 1 << 1);

pub trait Bag {
    fn path(&self) -> &Path;

    fn collider(&self) -> &ColliderShape;
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
            collider_type: ColliderType::Sensor.into(),
            shape: bag.collider().clone().into(),
            position: (transform.translation, transform.rotation).into(),
            flags: ColliderFlags {
                collision_groups: BAG_COLLIDER_GROUP,
                ..default()
            }
            .into(),
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
    collider: ColliderShape,
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

        let collider = ColliderShape::compound(vec![
            (
                Vec3::new(3., 2., 0.).into(),
                ColliderShape::cuboid(3., 2., 0.),
            ),
            (
                Vec3::new(1., 5., 0.).into(),
                ColliderShape::cuboid(1., 1., 0.),
            ),
            (
                Vec3::new(4.5, 4.5, 0.).into(),
                ColliderShape::cuboid(1.5, 0.5, 0.),
            ),
            (
                Vec3::new(5.5, 5.5, 0.).into(),
                ColliderShape::cuboid(0.5, 0.5, 0.),
            ),
        ]);

        Self { path, collider }
    }
}

impl Bag for Level1Bag {
    fn path(&self) -> &Path {
        &self.path
    }

    fn collider(&self) -> &ColliderShape {
        &self.collider
    }
}
