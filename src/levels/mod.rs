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
mod level10;

mod level17;
mod level2;
mod level8;
mod level9;
mod scoring;
mod transitions;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelTransitionPlugin);
        app.add_plugin(ScoringPlugin);

        app.add_plugin(Level1Plugin);

        app.add_system_to_stage(LevelSpawnStage, init_levels);
    }
}

const LEVELS: [fn(Commands, Res<DipsWindow>, EventWriter<PiecePlaced>); 6] = [
    level1::init_level,
    level2::init_level,
    level8::init_level,
    level9::init_level,
    level10::init_level,
    level17::init_level,
];

fn init_levels(
    mut level_started: EventReader<LevelStarted>,
    commands: Commands,
    // TODO shouldn't need after https://github.com/dimforge/bevy_rapier/issues/172
    placed_pieces: EventWriter<PiecePlaced>,
    dips_window: Res<DipsWindow>,
) {
    if let Some(started) = level_started.iter().last() {
        LEVELS[**started as usize](commands, dips_window, placed_pieces);
    }
}
