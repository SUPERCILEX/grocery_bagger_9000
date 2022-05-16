use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier3d::prelude::*;

use consts::*;
pub use consts::{DEG_180, DEG_90, DEG_MIRRORED};
pub use movement::{PiecePickedUp, PiecePlaced, Selectable};
pub use spawn::{NominoMarker, NominoSpawner};

use crate::nominos::movement::PieceMovementPlugin;

mod consts;
mod movement;
mod spawn;

pub const NOMINO_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b1,
    filters: 0b1,
};

pub const TETROMINOS: [Nomino; 5] = [
    Nomino::TetrominoStraight,
    Nomino::TetrominoSquare,
    Nomino::TetrominoT,
    Nomino::TetrominoL,
    Nomino::TetrominoSkew,
];

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PieceMovementPlugin);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Nomino {
    TrominoStraight,
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
            Nomino::TrominoStraight => &TROMINO_STRAIGHT_PATH,
            Nomino::TetrominoStraight => &TETROMINO_STRAIGHT_PATH,
            Nomino::TetrominoSquare => &TETROMINO_SQUARE_PATH,
            Nomino::TetrominoT => &TETROMINO_T_PATH,
            Nomino::TetrominoL => &TETROMINO_L_PATH,
            Nomino::TetrominoSkew => &TETROMINO_SKEW_PATH,
            Nomino::_Last => panic!("you shouldn't be here!"),
        }
    }

    fn collider(&self) -> &Collider {
        match self {
            Nomino::TrominoStraight => &TROMINO_STRAIGHT_COLLIDER,
            Nomino::TetrominoStraight => &TETROMINO_STRAIGHT_COLLIDER,
            Nomino::TetrominoSquare => &TETROMINO_SQUARE_COLLIDER,
            Nomino::TetrominoT => &TETROMINO_T_COLLIDER,
            Nomino::TetrominoL => &TETROMINO_L_COLLIDER,
            Nomino::TetrominoSkew => &TETROMINO_SKEW_COLLIDER,
            Nomino::_Last => panic!("you shouldn't be here!"),
        }
    }
}
