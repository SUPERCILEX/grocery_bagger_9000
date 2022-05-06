use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltInstance, Piece, PresetPiecesConveyorBelt},
    levels::{CurrentLevel, LevelLoaded},
    nomino_consts::{DEG_180, DEG_MIRRORED},
    nominos::Nomino,
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Pink;

pub struct Level5Plugin;

impl Plugin for Level5Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut current: ResMut<CurrentLevel>,
    mut level_initialized: EventWriter<LevelLoaded>,
    mut conveyor_belt: ResMut<ConveyorBeltInstance>,
    dips_window: Res<DipsWindow>,
) {
    if current.level != 4 || current.root.is_some() {
        return;
    }

    let root = commands
        .spawn_bundle(TransformBundle::default())
        .with_children(|parent| {
            // TODO keep these and the pieces' coordinates up-to-date
            parent.spawn_bag::<1>(Color::default(), &dips_window);

            **conveyor_belt = Some(Box::new(PresetPiecesConveyorBelt::new([
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
        .id();
    current.root = Some(root);
    level_initialized.send(LevelLoaded(root));
}
