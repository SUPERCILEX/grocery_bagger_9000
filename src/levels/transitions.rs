use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

use crate::{
    animations::AnimationEvent,
    conveyor_belt::BeltEmptyEvent,
    gb9000::{
        GameState::{LevelEnded, Playing},
        GroceryBagger9000,
    },
    levels::init::LevelInitLabel,
};

pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelLoaded>();
        app.add_event::<LevelFinishedEvent>();

        app.add_system_to_stage(
            CoreStage::PreUpdate,
            level_change_handler.before(LevelInitLabel),
        );
        app.add_system_to_stage(
            CoreStage::Last,
            transition_handler.label(LevelTransitionLabel),
        );
        app.add_system_to_stage(
            CoreStage::Last,
            level_unload_handler
                .after(transition_handler)
                .label(LevelTransitionLabel),
        );
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct LevelTransitionLabel;

pub struct LevelFinishedEvent;

#[derive(Debug, Default)]
pub struct LevelChangeFsm {
    belt_empty: bool,
    bag_offscreen: bool,
}

fn transition_handler(
    mut belt_empty_events: EventReader<BeltEmptyEvent>,
    mut bag_offscreen: EventReader<TweenCompleted>,
    mut level_finished: EventWriter<LevelFinishedEvent>,
    mut level_fsm: Local<LevelChangeFsm>,
) {
    if belt_empty_events.iter().count() > 0 {
        level_fsm.belt_empty = true;
    }
    for TweenCompleted { user_data, .. } in bag_offscreen.iter() {
        if *user_data & AnimationEvent::BAG_OFF_SCREEN.bits() != 0 && level_fsm.belt_empty {
            level_fsm.bag_offscreen = true;
        }
    }

    if level_fsm.belt_empty && level_fsm.bag_offscreen {
        level_finished.send(LevelFinishedEvent);
        *level_fsm = default();
    }
}

fn level_unload_handler(
    mut commands: Commands,
    mut gb9000: ResMut<GroceryBagger9000>,
    mut level_finished: EventReader<LevelFinishedEvent>,
) {
    if level_finished.iter().count() > 0 {
        if let Some(initialized) = gb9000.level_root {
            commands.entity(initialized).despawn_recursive();
            gb9000.level_root = None;
        }
        gb9000.state = LevelEnded;
    }
}

#[derive(Deref)]
pub struct LevelLoaded(pub Entity);

fn level_change_handler(
    mut commands: Commands,
    mut gb9000: ResMut<GroceryBagger9000>,
    mut prev_level: Local<u16>,
) {
    if *prev_level != gb9000.current_level {
        *prev_level = gb9000.current_level;

        if let Some(initialized) = gb9000.level_root {
            commands.entity(initialized).despawn_recursive();
            gb9000.level_root = None;
        }

        if let Some(initialized) = gb9000.menu_root {
            commands.entity(initialized).despawn_recursive();
            gb9000.menu_root = None;
            gb9000.state = Playing;
        }
    }
}
