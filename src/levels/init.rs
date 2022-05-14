use bevy::prelude::*;

use crate::levels::{CurrentLevel, LevelLoaded};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub enum InitSystem {
    LevelInit,
}

pub fn level_init_chrome(
    level_num: u16,
    mut current: ResMut<CurrentLevel>,
    mut level_loaded: EventWriter<LevelLoaded>,
    init: impl FnOnce() -> Entity,
) {
    if current.level != (level_num - 1) || current.root.is_some() {
        return;
    }
    let root = init();

    current.root = Some(root);
    level_loaded.send(LevelLoaded(root));
}
