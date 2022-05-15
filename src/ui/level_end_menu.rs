use bevy::{app::Plugin, prelude::*};

use crate::{
    levels::{
        CurrentLevel, CurrentScore, GameState::Playing, LevelFinishedEvent, LevelTransitionLabel,
    },
    App,
};

pub struct LevelEndMenuPlugin;

impl Plugin for LevelEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::Last,
            show_level_end_screen.after(LevelTransitionLabel),
        );
    }
}

fn show_level_end_screen(
    mut level_end: EventReader<LevelFinishedEvent>,
    _score: Res<CurrentScore>,
    mut level: ResMut<CurrentLevel>,
) {
    if level_end.iter().count() > 0 {
        // TODO: put playing update into callback for button
        level.state = Playing;
        level.level += 1;
    }
}
