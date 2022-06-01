use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, RandomPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::Nomino,
    window_management::DipsWindow,
};

const NUM_PIECES: usize = 9;
const LEVEL_COLOR: NominoColor = NominoColor::Pink;
const LEVEL_OMINOS: [Nomino; 3] = [
    Nomino::TrominoStraight,
    Nomino::TetrominoStraight,
    Nomino::TetrominoSquare,
];

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    asset_server: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "Some levels are randomly generated\nand may not have a perfect solution",
    );
    commands.spawn_bag(&dips_window, &game_speed, [BAG_SIZE_SMALL, BAG_SIZE_SMALL]);
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(RandomPiecesConveyorBelt::new(
            NUM_PIECES,
            LEVEL_OMINOS,
            [LEVEL_COLOR],
        )),
    );
}
