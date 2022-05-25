use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_LARGE},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::{Nomino, PiecePlaced, DEG_180, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Blue;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    _: EventWriter<PiecePlaced>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bag(&dips_window, [BAG_SIZE_LARGE]);
    spawn_text_tutorial(&mut commands, asset_server, "Bags come in different sizes");
    commands.spawn_belt(Box::new(PresetPiecesConveyorBelt::new([
        Piece {
            nomino: Nomino::TetrominoStraight,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
        Piece {
            nomino: Nomino::TetrominoL,
            color: LEVEL_COLOR,
            rotation: *DEG_180,
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
        Piece {
            nomino: Nomino::TetrominoSkew,
            color: LEVEL_COLOR,
            rotation: *DEG_MIRRORED,
        },
        Piece {
            nomino: Nomino::TetrominoL,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
        Piece {
            nomino: Nomino::TetrominoSquare,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
    ])));
}
