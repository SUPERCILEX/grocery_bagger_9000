use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::{
        init::{level_init_chrome, InitSystem::LevelInit},
        CurrentLevel, LevelLoaded,
    },
    nominos::{Nomino, DEG_180, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Pink;

pub struct Level5Plugin;

impl Plugin for Level5Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level.label(LevelInit));
    }
}

fn init_level(
    mut commands: Commands,
    current: ResMut<CurrentLevel>,
    level_loaded: EventWriter<LevelLoaded>,
    dips_window: Res<DipsWindow>,
) {
    level_init_chrome(5, current, level_loaded, || {
        commands
            .spawn_bundle(TransformBundle::default())
            .with_children(|parent| {
                parent.spawn_bag::<1>(&dips_window);

                parent.spawn_belt(Box::new(PresetPiecesConveyorBelt::new([
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
            })
            .id()
    });
}
