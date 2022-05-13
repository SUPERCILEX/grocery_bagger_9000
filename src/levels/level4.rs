use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::{CurrentLevel, LevelLoaded},
    nominos::{Nomino, DEG_180, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Red;

pub struct Level4Plugin;

impl Plugin for Level4Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut current: ResMut<CurrentLevel>,
    mut level_initialized: EventWriter<LevelLoaded>,
    dips_window: Res<DipsWindow>,
) {
    if current.level != 3 || current.root.is_some() {
        return;
    }

    let root = commands
        .spawn_bundle(TransformBundle::default())
        .with_children(|parent| {
            parent.spawn_bag::<1>(&dips_window);

            parent.spawn_belt(Box::new(PresetPiecesConveyorBelt::new([
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
                    nomino: Nomino::TetrominoSkew,
                    color: LEVEL_COLOR,
                    rotation: Quat::IDENTITY,
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
                    nomino: Nomino::TetrominoSkew,
                    color: LEVEL_COLOR,
                    rotation: *DEG_MIRRORED,
                },
                Piece {
                    nomino: Nomino::TetrominoT,
                    color: LEVEL_COLOR,
                    rotation: Quat::IDENTITY,
                },
                Piece {
                    nomino: Nomino::TetrominoStraight,
                    color: LEVEL_COLOR,
                    rotation: Quat::IDENTITY,
                },
            ])));
        })
        .id();
    current.root = Some(root);
    level_initialized.send(LevelLoaded(root));
}
