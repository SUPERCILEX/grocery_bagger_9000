use bevy::prelude::*;

use level1::Level1Plugin;
use level2::Level2Plugin;
use level3::Level3Plugin;
use level4::Level4Plugin;
use level5::Level5Plugin;
use level6::Level6Plugin;
pub use scoring::CurrentScore;
use scoring::ScoringPlugin;
use transitions::LevelTransitionPlugin;
pub use transitions::{
    CurrentLevel, GameState, LevelFinishedEvent, LevelLoaded, LevelTransitionLabel,
};

mod init;
mod level1;
mod level2;
mod level3;
mod level4;
mod level5;
mod level6;
mod scoring;
mod transitions;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelTransitionPlugin);
        app.add_plugin(ScoringPlugin);

        app.add_plugin(Level1Plugin);
        app.add_plugin(Level2Plugin);
        app.add_plugin(Level3Plugin);
        app.add_plugin(Level4Plugin);
        app.add_plugin(Level5Plugin);
        app.add_plugin(Level6Plugin);
    }
}
