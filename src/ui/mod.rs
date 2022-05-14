use bevy::prelude::*;

use crate::ui::display_score::DisplayScorePlugin;

mod display_score;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DisplayScorePlugin);
    }
}
