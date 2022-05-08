use bevy::prelude::*;

use level1::Level1Plugin;
use level2::Level2Plugin;
use level3::Level3Plugin;
use level4::Level4Plugin;
use level5::Level5Plugin;
use scoring::ScoringPlugin;
use transitions::LevelTransitionPlugin;
pub use transitions::{CurrentLevel, LevelLoaded, LevelUnloaded};

mod level1;
mod level2;
mod level3;
mod level4;
mod level5;
mod scoring;
mod transitions;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>();
        app.add_event::<LevelLoaded>();
        app.add_event::<LevelUnloaded>();

        app.add_plugin(LevelTransitionPlugin);
        app.add_plugin(ScoringPlugin);

        app.add_plugin(Level1Plugin);
        app.add_plugin(Level2Plugin);
        app.add_plugin(Level3Plugin);
        app.add_plugin(Level4Plugin);
        app.add_plugin(Level5Plugin);
    }
}
