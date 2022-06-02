use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_LARGE, BAG_SIZE_SMALL},
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    nominos::{Nomino, NominoColor, DEG_MIRRORED},
    window_management::DipsWindow,
};

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    _: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, &game_speed, [BAG_SIZE_SMALL, BAG_SIZE_LARGE]);
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
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Orange),
            piece!(Nomino::TrominoStraight, NominoColor::Orange),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Orange),
            piece!(Nomino::TetrominoStraight, NominoColor::Orange),
            piece!(Nomino::TetrominoL, NominoColor::Orange),
            piece!(Nomino::TetrominoStraight, NominoColor::Orange),
            piece!(Nomino::TetrominoL, NominoColor::Orange),
            piece!(Nomino::TetrominoT, NominoColor::Orange),
            piece!(Nomino::TrominoL, NominoColor::Orange),
            piece!(Nomino::TetrominoT, NominoColor::Orange),
            piece!(Nomino::TetrominoL, NominoColor::Orange),
            piece!(Nomino::TrominoStraight, NominoColor::Orange),
            piece!(Nomino::TetrominoSkew, Mirrored, NominoColor::Orange),
            piece!(Nomino::TetrominoL, NominoColor::Orange),
            piece!(Nomino::TetrominoL, NominoColor::Orange),
            piece!(Nomino::TetrominoL, NominoColor::Orange),
            piece!(Nomino::TetrominoT, NominoColor::Orange),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Orange),
            piece!(Nomino::TrominoStraight, NominoColor::Orange),
            piece!(Nomino::TetrominoSkew, NominoColor::Orange),
            piece!(Nomino::TrominoStraight, NominoColor::Orange),
            piece!(Nomino::TrominoL, NominoColor::Orange),
        ])),
    );
}
