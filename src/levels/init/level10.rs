use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    nominos::{Nomino, PiecePlaced, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Gold;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    _: EventWriter<PiecePlaced>,
    _: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, [BAG_SIZE_SMALL]);
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
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
            Piece {
                nomino: Nomino::TetrominoStraight,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoL,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoL,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoStraight,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            },
        ])),
    );
}