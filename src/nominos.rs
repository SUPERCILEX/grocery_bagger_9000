use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier3d::prelude::*;
use paste::paste;

use crate::nomino_consts::*;

pub const NOMINO_COLLIDER_GROUP: InteractionGroups = InteractionGroups::new(1 << 0, 1 << 0);

pub trait Nomino {
    fn path(&self) -> &Path;

    fn collider(&self) -> &ColliderShape;
}

#[derive(Component, Default)]
pub struct NominoMarker;

#[derive(Bundle)]
pub struct NominoBundle {
    #[bundle]
    shape: ShapeBundle,
    #[bundle]
    collider: ColliderBundle,
    _marker: NominoMarker,
}

impl NominoBundle {
    pub fn new(nomino: impl Nomino, color: Color, transform: Transform) -> Self {
        let collider = ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            shape: nomino.collider().clone().into(),
            position: (transform.translation, transform.rotation).into(),
            flags: ColliderFlags {
                collision_groups: NOMINO_COLLIDER_GROUP,
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
            outline_mode: StrokeMode::new(Color::BLACK, 0.1),
        };

        Self {
            shape: GeometryBuilder::build_as(nomino.path(), draw_mode, transform),
            collider,
            _marker: default(),
        }
    }
}

macro_rules! nomino {
    ($type:ident, $shape:ident) => {
        paste! {
            pub struct [<$type $shape>];

            impl Default for [<$type $shape>] {
                fn default() -> Self {
                    Self
                }
            }

            impl Nomino for [<$type $shape>] {
                fn path(&self) -> &Path {
                    &[<$type:upper _ $shape:upper _PATH>]
                }

                fn collider(&self) -> &ColliderShape {
                    &*[<$type:upper _ $shape:upper _COLLIDER>]
                }
            }
        }
    };
}

nomino!(Tetromino, Straight);
nomino!(Tetromino, Square);
nomino!(Tetromino, T);
nomino!(Tetromino, L);
nomino!(Tetromino, Skew);
