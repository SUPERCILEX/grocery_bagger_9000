use bevy::prelude::*;

use init::*;
use level01::Level1Plugin;
use scoring::ScoringPlugin;
pub use scoring::{CurrentScore, ScoringSystems};
use transitions::LevelTransitionPlugin;
pub use transitions::{
    LevelFinished, LevelMarker, LevelSpawnStage, LevelStarted, LevelTransitionSystems,
};

use crate::{nominos::PiecePlaced, window_management::DipsWindow};

mod infinite_level;
mod init;
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

pub const LAST_LEVEL: usize = LEVELS.len() - 1;

fn init_levels(
    mut level_started: EventReader<LevelStarted>,
    commands: Commands,
    // TODO shouldn't need after https://github.com/dimforge/bevy_rapier/issues/172
    placed_pieces: EventWriter<PiecePlaced>,
    dips_window: Res<DipsWindow>,
    asset_server: Res<AssetServer>,
) {
    if let Some(started) = level_started.iter().last() {
        let level = **started as usize;
        if level < LEVELS.len() {
            LEVELS[level](commands, dips_window, placed_pieces, asset_server);
        } else {
            infinite_level::init_level(commands, dips_window);
        }
    }
}
