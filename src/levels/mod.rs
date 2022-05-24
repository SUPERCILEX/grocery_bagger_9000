use bevy::prelude::*;

use level10::Level10Plugin;
use level17::Level17Plugin;
use level8::Level8Plugin;
use level9::Level9Plugin;
use scoring::ScoringPlugin;
pub use scoring::{CurrentScore, ScoringSystems};
use transitions::LevelTransitionPlugin;
pub use transitions::{
    LevelFinished, LevelMarker, LevelSpawnStage, LevelStarted, LevelTransitionSystems,
};

mod level10;

mod level17;
mod level8;
mod level9;
mod scoring;
mod transitions;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelTransitionPlugin);
        app.add_plugin(ScoringPlugin);

        app.add_plugin(Level8Plugin);
        app.add_plugin(Level9Plugin);
        app.add_plugin(Level10Plugin);
        app.add_plugin(Level17Plugin);
    }
}
