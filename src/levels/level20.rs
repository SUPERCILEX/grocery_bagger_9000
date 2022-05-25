use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_LARGE},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, RandomPiecesConveyorBelt},
    nominos::{PiecePlaced, TETROMINOS},
    window_management::DipsWindow,
};

const NUM_PIECES: usize = 18;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    _: EventWriter<PiecePlaced>,
    _asset_server: Res<AssetServer>,
) {
    commands.spawn_bag(&dips_window, [BAG_SIZE_LARGE, BAG_SIZE_LARGE]);

    commands.spawn_belt(Box::new(RandomPiecesConveyorBelt::new(
        NUM_PIECES,
        TETROMINOS,
        [NominoColor::Blue, NominoColor::Green],
    )));
}
