use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_LARGE},
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    nominos::{Nomino, NominoColor, DEG_MIRRORED},
    robot::RobotSpawner,
    window_management::DipsWindow,
};

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    _: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, &game_speed, [BAG_SIZE_LARGE, BAG_SIZE_LARGE]);
    commands.spawn_robot();
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
            piece!(Nomino::TetrominoL, NominoColor::Gold),
            piece!(Nomino::TetrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TetrominoSkew, NominoColor::Gold),
            piece!(Nomino::TetrominoSkew, Mirrored, NominoColor::Gold),
            piece!(Nomino::TetrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Gold),
            piece!(Nomino::TetrominoT, NominoColor::Gold),
            piece!(Nomino::TetrominoT, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoStraight, NominoColor::Gold),
            piece!(Nomino::TetrominoL, NominoColor::Gold),
            piece!(Nomino::TetrominoT, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
        ])),
    );
}
