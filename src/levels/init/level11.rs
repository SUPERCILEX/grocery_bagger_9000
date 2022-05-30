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
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, [BAG_SIZE_LARGE]);
    spawn_text_tutorial(&mut commands, asset_server, "Bags come in different sizes");
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
            piece!(Nomino::TetrominoStraight),
            piece!(Nomino::TetrominoL, *DEG_180),
            piece!(Nomino::TetrominoSquare),
            piece!(Nomino::TetrominoStraight),
            piece!(Nomino::TetrominoSkew, *DEG_MIRRORED),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoSquare),
        ])),
    );
}
