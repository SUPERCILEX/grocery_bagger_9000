use bevy::prelude::*;

use crate::ui::{hud::HudPlugin, level_end_menu::LevelEndMenuPlugin};

mod consts;
mod hud;
mod level_end_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HudPlugin);
        app.add_plugin(LevelEndMenuPlugin);
    }
}
