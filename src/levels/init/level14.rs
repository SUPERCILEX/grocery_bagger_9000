use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_LARGE, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, RandomPiecesConveyorBelt},
    nominos::Nomino,
    window_management::DipsWindow,
};

const NUM_PIECES: usize = 18;
const LEVEL_COLOR: NominoColor = NominoColor::Orange;
const LEVEL_OMINOS: [Nomino; 5] = [
    Nomino::TrominoStraight,
    Nomino::TrominoL,
    Nomino::TetrominoStraight,
    Nomino::TetrominoSquare,
    Nomino::TetrominoL,
];

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
    commands.spawn_belt(
        dips_window,
        Box::new(RandomPiecesConveyorBelt::new(
            NUM_PIECES,
            LEVEL_OMINOS,
            [LEVEL_COLOR],
        )),
    );
}
