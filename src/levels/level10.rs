use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_LARGE},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::{transitions::LevelSpawnStage, LevelMarker, LevelStarted},
    nominos::{Nomino, DEG_180, DEG_MIRRORED},
    robot::{RobotMarker, RobotTiming},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Pink;

pub struct Level10Plugin;

impl Plugin for Level10Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(LevelSpawnStage, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut level_started: EventReader<LevelStarted>,
    dips_window: Res<DipsWindow>,
) {
    if !level_started.iter().last().map(|l| **l).contains(&9) {
        return;
    }

    commands.spawn_bag(&dips_window, [BAG_SIZE_LARGE]);

    // TODO remove
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelMarker)
        .insert(RobotTiming::default())
        .insert(RobotMarker);

    commands.spawn_belt(Box::new(PresetPiecesConveyorBelt::new([
        Piece {
            nomino: Nomino::TetrominoL,
            color: LEVEL_COLOR,
            rotation: *DEG_MIRRORED * *DEG_180,
        },
        Piece {
            nomino: Nomino::TetrominoL,
            color: LEVEL_COLOR,
            rotation: *DEG_MIRRORED * *DEG_180,
        },
        Piece {
            nomino: Nomino::TetrominoSkew,
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
            nomino: Nomino::TetrominoL,
            color: LEVEL_COLOR,
            rotation: *DEG_180,
        },
        Piece {
            nomino: Nomino::TetrominoT,
            color: LEVEL_COLOR,
            rotation: *DEG_180,
        },
        Piece {
            nomino: Nomino::TetrominoT,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
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
            nomino: Nomino::TetrominoT,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
        Piece {
            nomino: Nomino::TetrominoSkew,
            color: LEVEL_COLOR,
            rotation: *DEG_MIRRORED,
        },
        Piece {
            nomino: Nomino::TetrominoT,
            color: LEVEL_COLOR,
            rotation: *DEG_180,
        },
        Piece {
            nomino: Nomino::TetrominoL,
            color: LEVEL_COLOR,
            rotation: *DEG_MIRRORED * *DEG_180,
        },
        Piece {
            nomino: Nomino::TetrominoL,
            color: LEVEL_COLOR,
            rotation: *DEG_MIRRORED * *DEG_180,
        },
        Piece {
            nomino: Nomino::TetrominoT,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
        Piece {
            nomino: Nomino::TetrominoSkew,
            color: LEVEL_COLOR,
            rotation: *DEG_MIRRORED,
        },
        Piece {
            nomino: Nomino::TetrominoT,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
    ])));
}
