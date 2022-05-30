use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    nominos::{Nomino, DEG_MIRRORED},
    window_management::DipsWindow,
};

pub fn init_level(mut commands: Commands, dips_window: Res<DipsWindow>, _: Res<AssetServer>) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(
        &dips_window,
        [BAG_SIZE_SMALL, BAG_SIZE_SMALL, BAG_SIZE_SMALL],
    );
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    macro_rules! piece {
        ($nomino:expr, $color:expr) => {{
            Piece {
                nomino: $nomino,
                color: $color,
                rotation: Quat::IDENTITY,
            }
        }};

        ($nomino:expr, Mirrored, $color:expr) => {{
            Piece {
                nomino: $nomino,
                color: $color,
                rotation: *DEG_MIRRORED,
            }
        }};
    }

    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            piece!(Nomino::TrominoStraight, NominoColor::Pink),
            piece!(Nomino::TetrominoSkew, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Pink),
            piece!(Nomino::TetrominoL, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Pink),
            piece!(Nomino::TrominoStraight, NominoColor::Pink),
            piece!(Nomino::TetrominoL, NominoColor::Blue),
            piece!(Nomino::TrominoStraight, NominoColor::Pink),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Red),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Blue),
            piece!(Nomino::TetrominoSquare, NominoColor::Gold),
            piece!(Nomino::TetrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoSquare, NominoColor::Gold),
            piece!(Nomino::TetrominoSkew, NominoColor::Blue),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Green),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TetrominoStraight, NominoColor::Blue),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Red),
            piece!(Nomino::TetrominoStraight, NominoColor::Blue),
            piece!(Nomino::TrominoStraight, NominoColor::Pink),
            piece!(Nomino::TetrominoT, NominoColor::Pink),
            piece!(Nomino::TetrominoStraight, NominoColor::Blue),
            piece!(Nomino::TrominoStraight, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Green),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Pink),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Green),
            piece!(Nomino::TrominoL, NominoColor::Pink),
            piece!(Nomino::TetrominoL, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Green),
            piece!(Nomino::TrominoStraight, NominoColor::Red),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Red),
            piece!(Nomino::TetrominoT, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Red),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TetrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoT, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Red),
            piece!(Nomino::TrominoL, NominoColor::Gold),
        ])),
    );
}
