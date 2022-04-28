use bevy::prelude::*;

use crate::level1::Level1Plugin;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>();
        app.add_plugin(Level1Plugin);
    }
}

#[derive(Default)]
pub struct CurrentLevel {
    pub level: u16,
}
