use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    nominos::{Nomino, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Gold;

pub fn init_level(mut commands: Commands, dips_window: Res<DipsWindow>, _: Res<AssetServer>) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, [BAG_SIZE_SMALL]);
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    macro_rules! piece {
        (mirrored $nomino:expr) => {{
            Piece {
                nomino: $nomino,
                color: LEVEL_COLOR,
                rotation: *DEG_MIRRORED,
            }
        }};

        ($nomino:expr) => {{
            Piece {
                nomino: $nomino,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            }
        }};
    }

    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoSquare),
            piece!(mirrored Nomino::TetrominoSkew),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoSquare),
            piece!(Nomino::TetrominoStraight),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoStraight),
        ])),
    );
}
