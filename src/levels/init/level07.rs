use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::{Nomino, PiecePlaced},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Gold;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    _: EventWriter<PiecePlaced>,
    asset_server: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, [BAG_SIZE_SMALL]);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "…try a different item arrangement\nwith these same pieces for a higher score",
    )
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
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TrominoL,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            },
        ])),
    );
}
