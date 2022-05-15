use bevy::prelude::*;

use crate::ui::{display_score::DisplayScorePlugin, level_end_menu::LevelEndMenuPlugin};

mod display_score;
mod level_end_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DisplayScorePlugin);
        app.add_plugin(LevelEndMenuPlugin);
    }
}
