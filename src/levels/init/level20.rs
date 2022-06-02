use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
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
    commands.spawn_bag(
        &dips_window,
        &game_speed,
        [BAG_SIZE_SMALL, BAG_SIZE_SMALL, BAG_SIZE_SMALL],
    );
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
            piece!(Nomino::TetrominoStraight, NominoColor::Green),
            piece!(Nomino::TetrominoStraight, NominoColor::Green),
            piece!(Nomino::TrominoStraight, NominoColor::Blue),
            piece!(Nomino::TetrominoStraight, NominoColor::Green),
            piece!(Nomino::TrominoL, NominoColor::Blue),
            piece!(Nomino::TetrominoStraight, NominoColor::Green),
            piece!(Nomino::TetrominoT, NominoColor::Blue),
            piece!(Nomino::TrominoStraight, NominoColor::Blue),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Blue),
            piece!(Nomino::TetrominoL, NominoColor::Blue),
            piece!(Nomino::TrominoL, NominoColor::Blue),
            piece!(Nomino::TetrominoT, NominoColor::Blue),
            piece!(Nomino::TetrominoSquare, NominoColor::Blue),
            piece!(Nomino::TetrominoT, NominoColor::Blue),
            piece!(Nomino::TetrominoT, NominoColor::Blue),
            piece!(Nomino::TetrominoL, NominoColor::Blue),
            piece!(Nomino::TrominoL, NominoColor::Pink),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Blue),
            piece!(Nomino::TrominoL, NominoColor::Pink),
            piece!(Nomino::TrominoL, NominoColor::Pink),
            piece!(Nomino::TrominoL, NominoColor::Pink),
        ])),
    );
}
