use bevy::prelude::*;

use crate::{
    bags::BagContainerSpawner,
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::{LevelInitLabel, LevelStarted},
    nominos::{Nomino, DEG_180, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Blue;

pub struct Level3Plugin;

impl Plugin for Level3Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level.label(LevelInitLabel));
    }
}

fn init_level(
    mut commands: Commands,
    mut level_started: EventReader<LevelStarted>,
    dips_window: Res<DipsWindow>,
) {
    if !level_started.iter().last().map(|l| **l).contains(&2) {
        return;
    }

    commands.spawn_bag::<1>(&dips_window);

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
