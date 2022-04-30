use std::lazy::SyncLazy;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        tess::{math::Point, path::path::Builder},
        *,
    },
};
use bevy_rapier3d::prelude::*;

pub const BAG_COLLIDER_GROUP: InteractionGroups = InteractionGroups::new(0b10, 0b10);
pub const BAG_BOUNDARY_COLLIDER_GROUP: InteractionGroups = InteractionGroups::new(0b100, 0b100);

static BAG_PATH: SyncLazy<Path> = SyncLazy::new(|| {
    let mut b = Builder::with_capacity(4, 4);

    b.begin(Point::new(-3., 3.));
    b.line_to(Point::new(-3., -3.));
    b.line_to(Point::new(3., -3.));
    b.line_to(Point::new(3., 3.));
    b.end(false);

    Path(b.build())
});

static MAIN_BAG_COLLIDER: SyncLazy<ColliderShape> =
    SyncLazy::new(|| ColliderShape::cuboid(3., 3., 0.));

static BOUNDARY_BAG_COLLIDER: SyncLazy<ColliderShape> = SyncLazy::new(|| {
    ColliderShape::compound(vec![
        (
            Vec3::new(-3., 0., 0.).into(),
            ColliderShape::cuboid(0.009, 3., 0.),
        ),
        (
            Vec3::new(0., -3., 0.).into(),
            ColliderShape::cuboid(3., 0.009, 0.),
        ),
        (
            Vec3::new(3., 0., 0.).into(),
            ColliderShape::cuboid(0.009, 3., 0.),
        ),
    ])
});

#[derive(Bundle)]
pub struct BagBundle {
    #[bundle]
    shape: ShapeBundle,
    #[bundle]
    collider: ColliderBundle,
}

pub trait BagUtils {
    fn spawn_bag(&mut self, color: Color, transform: Transform);
}

impl<'w, 's, 'a> BagUtils for ChildBuilder<'w, 's, 'a> {
    fn spawn_bag(&mut self, color: Color, transform: Transform) {
        let main_collider = ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            shape: MAIN_BAG_COLLIDER.clone().into(),
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

        self.spawn_bundle(BagBundle {
            shape: GeometryBuilder::build_as(&*BAG_PATH, draw_mode, transform),
            collider: main_collider,
        })
        .with_children(|parent| {
            let boundery_collider = ColliderBundle {
                collider_type: ColliderType::Sensor.into(),
                shape: BOUNDARY_BAG_COLLIDER.clone().into(),
                position: (transform.translation, transform.rotation).into(),
                flags: ColliderFlags {
                    collision_groups: BAG_BOUNDARY_COLLIDER_GROUP,
                    ..default()
                }
                .into(),
                ..default()
            };

            parent.spawn_bundle(boundery_collider);
        });
    }
}

pub trait BagSnapper<T> {
    fn snap_to_grid(&self) -> T;
}

#[derive(Deref)]
pub struct BagCoord(pub f32);

impl BagSnapper<f32> for BagCoord {
    fn snap_to_grid(&self) -> f32 {
        self.round() + 0.5
    }
}
