// TODO https://github.com/rust-lang/rust-clippy/issues/6902
#![allow(clippy::use_self)]

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier3d::prelude::*;
use num_derive::FromPrimitive;

use consts::*;
pub use consts::{DEG_180, DEG_90, DEG_MIRRORED};
use movement::PieceMovementPlugin;
pub use movement::{
    OutOfBagPlacement, PiecePickedUp, PiecePlaced, PieceSystems, Selectable, Selected,
};
pub use spawn::{NominoMarker, NominoSpawner};

mod consts;
mod movement;
mod spawn;

pub const NOMINO_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b1,
    filters: 0b1,
};

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PieceMovementPlugin);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
pub enum Nomino {
    TrominoStraight,
    TrominoL,
    TetrominoStraight,
    TetrominoSquare,
    TetrominoT,
    TetrominoL,
    TetrominoSkew,
    _Last,
}

impl Nomino {
    fn path(&self) -> &Path {
        match self {
            Self::TrominoStraight => &TROMINO_STRAIGHT_PATH,
            Self::TrominoL => &TROMINO_L_PATH,
            Self::TetrominoStraight => &TETROMINO_STRAIGHT_PATH,
            Self::TetrominoSquare => &TETROMINO_SQUARE_PATH,
            Self::TetrominoT => &TETROMINO_T_PATH,
            Self::TetrominoL => &TETROMINO_L_PATH,
            Self::TetrominoSkew => &TETROMINO_SKEW_PATH,
            Self::_Last => panic!("you shouldn't be here!"),
        }
    }

    fn collider(&self) -> &Collider {
        match self {
            Self::TrominoStraight => &TROMINO_STRAIGHT_COLLIDER,
            Self::TrominoL => &TROMINO_L_COLLIDER,
            Self::TetrominoStraight => &TETROMINO_STRAIGHT_COLLIDER,
            Self::TetrominoSquare => &TETROMINO_SQUARE_COLLIDER,
            Self::TetrominoT => &TETROMINO_T_COLLIDER,
            Self::TetrominoL => &TETROMINO_L_COLLIDER,
            Self::TetrominoSkew => &TETROMINO_SKEW_COLLIDER,
            Self::_Last => panic!("you shouldn't be here!"),
        }
    }
}
