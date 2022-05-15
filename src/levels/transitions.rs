use bevy::prelude::*;

pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>();
        app.add_event::<LevelLoaded>();

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

fn level_change_handler(
    mut commands: Commands,
    mut current: ResMut<CurrentLevel>,
    mut prev_level: Local<u16>,
) {
    if *prev_level != current.level {
        *prev_level = current.level;
        if let Some(initialized) = current.root {
            commands.entity(initialized).despawn_recursive();
            current.root = None;
        }
    }
}
