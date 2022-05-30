use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::Nomino,
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Green;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    asset_server: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, [BAG_SIZE_SMALL]);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "Try to completely fill the bags",
    );
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
    }

    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            piece!(Nomino::TetrominoSquare),
            piece!(Nomino::TetrominoSquare),
            piece!(Nomino::TetrominoStraight),
        ])),
    );
}
