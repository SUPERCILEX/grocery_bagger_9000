use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_LARGE},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::LevelMarker,
    nominos::{Nomino, DEG_180, DEG_MIRRORED},
    robot::{RobotMarker, RobotTiming},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Pink;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    _: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, &game_speed, [BAG_SIZE_LARGE]);

    // TODO remove
    #[cfg(debug_assertions)]
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelMarker)
        .insert(RobotTiming::default())
        .insert(RobotMarker);
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    macro_rules! piece {
        ($nomino:expr) => {{
            Piece {
                nomino: $nomino,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            }
        }};

        ($nomino:expr, $rotation:expr) => {{
            Piece {
                nomino: $nomino,
                color: LEVEL_COLOR,
                rotation: $rotation,
            }
        }};
    }

    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            piece!(Nomino::TetrominoL, *DEG_MIRRORED * *DEG_180),
            piece!(Nomino::TetrominoL, *DEG_MIRRORED * *DEG_180),
            piece!(Nomino::TetrominoSkew),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoL, *DEG_180),
            piece!(Nomino::TetrominoT, *DEG_180),
            piece!(Nomino::TetrominoT),
            piece!(Nomino::TetrominoSquare),
            piece!(Nomino::TetrominoSquare),
            piece!(Nomino::TetrominoT),
            piece!(Nomino::TetrominoSkew, *DEG_MIRRORED),
            piece!(Nomino::TetrominoT, *DEG_180),
            piece!(Nomino::TetrominoL, *DEG_MIRRORED * *DEG_180),
            piece!(Nomino::TetrominoL, *DEG_MIRRORED * *DEG_180),
            piece!(Nomino::TetrominoT),
            piece!(Nomino::TetrominoSkew, *DEG_MIRRORED),
            piece!(Nomino::TetrominoT),
        ])),
    );
}
