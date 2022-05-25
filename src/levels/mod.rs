use bevy::prelude::*;

use level1::Level1Plugin;
use scoring::ScoringPlugin;
pub use scoring::{CurrentScore, ScoringSystems};
use transitions::LevelTransitionPlugin;
pub use transitions::{
    LevelFinished, LevelMarker, LevelSpawnStage, LevelStarted, LevelTransitionSystems,
};

use crate::{nominos::PiecePlaced, window_management::DipsWindow};

mod level1;
mod level11;
mod level12;
mod level13;
mod level2;
mod level20;
mod level3;
mod level4;
mod level5;
mod level6;
mod level7;
mod scoring;
mod transitions;
mod tutorials;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelTransitionPlugin);
        app.add_plugin(ScoringPlugin);

        app.add_plugin(Level1Plugin);

        app.add_system_to_stage(LevelSpawnStage, init_levels);
    }
}

const LEVELS: [fn(Commands, Res<DipsWindow>, EventWriter<PiecePlaced>, Res<AssetServer>); 11] = [
    level1::init_level,
    level2::init_level,
    level3::init_level,
    level4::init_level,
    level5::init_level,
    level6::init_level,
    level7::init_level,
    level11::init_level,
    level12::init_level,
    level13::init_level,
    level20::init_level,
];

fn init_levels(
    mut level_started: EventReader<LevelStarted>,
    commands: Commands,
    // TODO shouldn't need after https://github.com/dimforge/bevy_rapier/issues/172
    placed_pieces: EventWriter<PiecePlaced>,
    dips_window: Res<DipsWindow>,
    asset_server: Res<AssetServer>,
) {
    if let Some(started) = level_started.iter().last() {
        LEVELS[**started as usize](commands, dips_window, placed_pieces, asset_server);
    }
}
