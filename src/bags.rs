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

pub const BAG_COLLIDER_GROUP: InteractionGroups = InteractionGroups::new(0b10, 0b10);

#[derive(Bundle)]
pub struct BagBundle {
    #[bundle]
    shape: ShapeBundle,
    #[bundle]
    collider: ColliderBundle,
}

impl BagBundle {
    pub fn new(color: Color, transform: Transform) -> Self {
        // TODO extract into constants
        let mut b = Builder::with_capacity(4, 4);

        b.begin(Point::new(0., 6.));
        b.line_to(Point::new(0., 0.));
        b.line_to(Point::new(6., 0.));
        b.line_to(Point::new(6., 6.));
        b.end(false);

        let path = Path(b.build());

        // TODO make 1 big plus polyline for edges
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

        let collider = ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            shape: collider.into(),
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
            shape: GeometryBuilder::build_as(&path, draw_mode, transform),
            collider,
        }
    }
}
