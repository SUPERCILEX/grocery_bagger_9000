use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    nominos::{Nomino, PiecePlaced},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Green;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    _: EventWriter<PiecePlaced>,
) {
    commands.spawn_belt(Box::new(PresetPiecesConveyorBelt::new([
        Piece {
            nomino: Nomino::TetrominoSquare,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
        Piece {
            nomino: Nomino::TetrominoSquare,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
        Piece {
            nomino: Nomino::TetrominoStraight,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
    ])));

    commands.spawn_bag(&dips_window, [BAG_SIZE_SMALL]);
}
