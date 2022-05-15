use bevy::prelude::*;

use crate::{
    gb9000::{GameState::Playing, GroceryBagger9000},
    levels::LevelLoaded,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct LevelInitLabel;

pub fn level_init_chrome(
    level_num: u16,
    mut gb9000: ResMut<GroceryBagger9000>,
    mut level_loaded: EventWriter<LevelLoaded>,
    init: impl FnOnce() -> Entity,
) {
    if gb9000.current_level == (level_num - 1)
        && gb9000.level_root.is_none()
        && gb9000.state == Playing
    {
        let root = init();
        gb9000.level_root = Some(root);
        level_loaded.send(LevelLoaded(root));
    }
}
