use bevy::prelude::*;

use crate::levels::{CurrentLevel, GameState::Playing, LevelLoaded};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct LevelInitLabel;

pub fn level_init_chrome(
    level_num: u16,
    mut current: ResMut<CurrentLevel>,
    mut level_loaded: EventWriter<LevelLoaded>,
    init: impl FnOnce() -> Entity,
) {
    if current.level == (level_num - 1) && current.root.is_none() && current.state == Playing {
        let root = init();
        current.root = Some(root);
        level_loaded.send(LevelLoaded(root));
    }
}
