use bevy::prelude::*;

use crate::level1::Level1Plugin;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>();
        app.add_event::<LevelLoaded>();
        app.add_event::<LevelUnloaded>();

        app.add_plugin(Level1Plugin);

        app.add_system_to_stage(CoreStage::First, level_change_handler);
    }
}

#[derive(Default)]
pub struct CurrentLevel {
    // TODO add state enum such as PLAYING, LEVEL_ENDED
    pub level: u16,
    pub root: Option<Entity>,
}

#[derive(Deref)]
pub struct LevelLoaded(pub Entity);

pub struct LevelUnloaded;

fn level_change_handler(
    mut commands: Commands,
    mut current: ResMut<CurrentLevel>,
    mut prev_level: Local<u16>,
    mut level_unloaded: EventWriter<LevelUnloaded>,
) {
    if *prev_level != current.level {
        *prev_level = current.level;
        if let Some(initialized) = current.root {
            commands.entity(initialized).despawn_recursive();
            current.root = None;
            level_unloaded.send(LevelUnloaded);
        }
    }
}
