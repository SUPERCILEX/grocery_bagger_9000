use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use wasm_bindgen::prelude::*;

use crate::{
    gb9000::GroceryBagger9000,
    levels::{LevelFinished, LevelLoaded},
    nominos::PiecePlaced,
};

pub struct AnalyticsPlugin;

impl Plugin for AnalyticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(log_level_start);
        app.add_system(log_level_end);
        app.add_system(log_piece_placed);
    }
}

#[derive(Copy, Clone)]
enum ActionIds {
    PiecePlaced = 0,
}

#[derive(Copy, Clone)]
enum VersionIds {
    FamilyFriends = 0,
}

fn log_level_start(
    mut level_start: EventReader<LevelLoaded>,
    thread_pool: Res<AsyncComputeTaskPool>,
    gb9000: Res<GroceryBagger9000>,
) {
    if level_start.iter().count() == 0 {
        return;
    }

    let level_no = gb9000.current_level as u32;
    thread_pool
        .spawn(async move {
            logLevelStart(level_no);
        })
        .detach();
}

fn log_level_end(
    mut level_end: EventReader<LevelFinished>,
    thread_pool: Res<AsyncComputeTaskPool>,
    gb9000: Res<GroceryBagger9000>,
) {
    if level_end.iter().count() == 0 {
        return;
    }

    let level_no = gb9000.current_level as u32;
    thread_pool
        .spawn(async move {
            logLevelEnd(level_no);
        })
        .detach();
}

fn log_piece_placed(
    mut piece_placed: EventReader<PiecePlaced>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    for _ in piece_placed.iter() {
        thread_pool
            .spawn(async {
                logLevelAction(ActionIds::PiecePlaced as u32);
            })
            .detach();
    }
}

#[wasm_bindgen]
extern "C" {
    fn init(version_no: u32);

    fn logLevelAction(action_id: u32);

    fn logActionWithNoLevel(action_id: u32);

    fn logLevelStart(level_id: u32);

    fn logLevelEnd(level_id: u32);
}
