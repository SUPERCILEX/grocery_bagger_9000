use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier3d::prelude::*;
use paste::paste;

use crate::nomino_consts::*;

pub trait Nomino {
    fn path(&self) -> &Path;

    fn vertices(&self) -> &'static [Point<Real>];
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
    pub fn new(nomino: impl Nomino, draw_mode: DrawMode, transform: Transform) -> Self {
        let collider = ColliderBundle {
            shape: ColliderShape::convex_hull(nomino.vertices())
                .unwrap()
                .into(),
            position: (transform.translation, transform.rotation).into(),
            ..Default::default()
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

                fn vertices(&self) -> &'static [Point<Real>] {
                    &*[<$type:upper _ $shape:upper _VERTICES>]
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
