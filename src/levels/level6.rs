use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_LARGE},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, RandomPiecesConveyorBelt},
    levels::{transitions::LevelSpawnStage, LevelStarted},
    nominos::TETROMINOS,
    window_management::DipsWindow,
};

const NUM_PIECES: usize = 18;

pub struct Level6Plugin;

impl Plugin for Level6Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(LevelSpawnStage, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut level_started: EventReader<LevelStarted>,
    dips_window: Res<DipsWindow>,
) {
    if !level_started.iter().last().map(|l| **l).contains(&5) {
        return;
    }

    commands.spawn_bag(&dips_window, [BAG_SIZE_LARGE, BAG_SIZE_LARGE]);

    commands.spawn_belt(Box::new(RandomPiecesConveyorBelt::new(
        NUM_PIECES,
        TETROMINOS,
        [NominoColor::Blue, NominoColor::Green],
    )));
}
