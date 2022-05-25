use bevy::prelude::*;

use level01::Level1Plugin;
use scoring::ScoringPlugin;
pub use scoring::{CurrentScore, ScoringSystems};
use transitions::LevelTransitionPlugin;
pub use transitions::{
    LevelFinished, LevelMarker, LevelSpawnStage, LevelStarted, LevelTransitionSystems,
};

use crate::{nominos::PiecePlaced, window_management::DipsWindow};

mod level01;
mod level02;
mod level03;
mod level04;
mod level05;
mod level06;
mod level07;
mod level08;
mod level09;
mod level10;
mod level11;
mod level12;
mod level13;
mod level14;
mod level15;
mod level16;
mod level17;
mod level18;
mod level19;
mod level20;
mod level21;
mod level22;
mod scoring;
mod transitions;
mod tutorials;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelTransitionPlugin);
        app.add_plugin(ScoringPlugin);

        app.add_plugin(Level1Plugin);

        app.add_system_to_stage(LevelSpawnStage, init_levels);
    }
}

const LEVELS: &[fn(Commands, Res<DipsWindow>, EventWriter<PiecePlaced>, Res<AssetServer>)] = &[
    level01::init_level,
    level02::init_level,
    level03::init_level,
    level04::init_level,
    level05::init_level,
    level06::init_level,
    level07::init_level,
    level08::init_level,
    level09::init_level,
    level10::init_level,
    level11::init_level,
    level12::init_level,
    level13::init_level,
    level14::init_level,
    level15::init_level,
    level16::init_level,
    level17::init_level,
    level18::init_level,
    level19::init_level,
    level20::init_level,
    level21::init_level,
    level22::init_level,
];

fn init_levels(
    mut level_started: EventReader<LevelStarted>,
    commands: Commands,
    // TODO shouldn't need after https://github.com/dimforge/bevy_rapier/issues/172
    placed_pieces: EventWriter<PiecePlaced>,
    dips_window: Res<DipsWindow>,
    asset_server: Res<AssetServer>,
) {
    if let Some(started) = level_started.iter().last() {
        LEVELS[**started as usize](commands, dips_window, placed_pieces, asset_server);
    }
}
