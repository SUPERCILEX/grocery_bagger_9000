use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, RandomPiecesConveyorBelt},
    nominos::Nomino,
    window_management::DipsWindow,
};

const NUM_PIECES: usize = 18;
const LEVEL_OMINOS: [Nomino; 3] = [
    Nomino::TrominoStraight,
    Nomino::TetrominoStraight,
    Nomino::TetrominoSquare,
];

pub fn init_level(mut commands: Commands, dips_window: Res<DipsWindow>, _: Res<AssetServer>) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, [BAG_SIZE_SMALL, BAG_SIZE_SMALL]);
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(RandomPiecesConveyorBelt::new(
            NUM_PIECES,
            LEVEL_OMINOS,
            [NominoColor::Pink, NominoColor::Blue],
        )),
    );
}
