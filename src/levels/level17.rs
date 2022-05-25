use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_LARGE},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, RandomPiecesConveyorBelt},
    nominos::{PiecePlaced, OMINOS},
    window_management::DipsWindow,
};

const NUM_PIECES: usize = 18;
const LEVEL_COLOR: NominoColor = NominoColor::Red;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    _: EventWriter<PiecePlaced>,
    _: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, [BAG_SIZE_LARGE, BAG_SIZE_LARGE]);
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(RandomPiecesConveyorBelt::new(
            NUM_PIECES,
            OMINOS,
            [LEVEL_COLOR],
        )),
    );
}
