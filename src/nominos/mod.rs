use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier3d::prelude::*;
use num_derive::FromPrimitive;

use consts::*;
pub use consts::{DEG_180, DEG_90, DEG_MIRRORED};
use movement::PieceMovementPlugin;
pub use movement::{
    AttemptedPlacement, PiecePickedUp, PiecePlaced, PieceSystems, Selectable, Selected,
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
            Nomino::TrominoStraight => &TROMINO_STRAIGHT_PATH,
            Nomino::TrominoL => &TROMINO_L_PATH,
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
            Nomino::TrominoL => &TROMINO_L_COLLIDER,
            Nomino::TetrominoStraight => &TETROMINO_STRAIGHT_COLLIDER,
            Nomino::TetrominoSquare => &TETROMINO_SQUARE_COLLIDER,
            Nomino::TetrominoT => &TETROMINO_T_COLLIDER,
            Nomino::TetrominoL => &TETROMINO_L_COLLIDER,
            Nomino::TetrominoSkew => &TETROMINO_SKEW_COLLIDER,
            Nomino::_Last => panic!("you shouldn't be here!"),
        }
    }
}
