use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::levels::LevelStarted;

pub fn run_if_level_started(mut level_loaded: EventReader<LevelStarted>) -> ShouldRun {
    if level_loaded.iter().count() > 0 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
