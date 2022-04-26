use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use paste::paste;

use crate::nomino_consts::*;

pub trait Nomino {
    fn path(&self) -> &Path;

    fn bounding_boxes(&self) -> &'static [Rect<f32>];
}

#[derive(Component, Default)]
pub struct NominoMarker;

#[derive(Component, Deref)]
pub struct BoundingBoxes(&'static [Rect<f32>]);

impl BoundingBoxes {
    pub fn contains(&self, base_position: &Transform, mouse_position: Vec2) -> bool {
        let t = base_position.translation.truncate();
        if !mouse_position.cmpge(t).all() {
            return false;
        }
        let t = mouse_position - t;

        for bound in self.0 {
            if t.cmpge(bound.start()).all() && t.cmple(bound.end()).all() {
                return true;
            }
        }
        false
    }
}

trait RectHelpers {
    fn start(&self) -> Vec2;

    fn end(&self) -> Vec2;
}

impl RectHelpers for Rect<f32> {
    #[inline]
    fn start(&self) -> Vec2 {
        Vec2::new(self.left, self.bottom)
    }

    #[inline]
    fn end(&self) -> Vec2 {
        Vec2::new(self.right, self.top)
    }
}

#[derive(Bundle)]
pub struct NominoBundle {
    #[bundle]
    shape: ShapeBundle,
    bounding_boxes: BoundingBoxes,
    _marker: NominoMarker,
}

impl NominoBundle {
    pub fn new(nomino: impl Nomino, draw_mode: DrawMode, transform: Transform) -> Self {
        Self {
            shape: GeometryBuilder::build_as(nomino.path(), draw_mode, transform),
            bounding_boxes: BoundingBoxes(nomino.bounding_boxes()),
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

                fn bounding_boxes(&self) -> &'static [Rect<f32>] {
                    [<$type:upper _ $shape:upper _BOUNDING_BOXES>]
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
