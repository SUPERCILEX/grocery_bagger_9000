use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::{Nomino, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Red;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    asset_server: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, &game_speed, [BAG_SIZE_SMALL]);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "Bags are worth more the fuller they are,\nbut sometimes you won't be able to fill\na bag completely…",
    );
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            Piece {
                nomino: Nomino::TetrominoSquare,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoStraight,
                color: LEVEL_COLOR,
                rotation: *DEG_MIRRORED,
            },
        ])),
    );
}
