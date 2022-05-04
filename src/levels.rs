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
    // TODO add state enum such as PLAYING, LEVEL_ENDED
    pub level: u16,
    pub root: Option<Entity>,
}
