use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

use crate::{
    animations::AnimationEvent,
    conveyor_belt::BeltEmptyEvent,
    gb9000::{GameState::LevelEnded, GroceryBagger9000},
    nominos::PiecePlaced,
};

pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelLoaded>();
        app.add_event::<LevelFinished>();

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

pub struct LevelFinished;

#[derive(Deref)]
pub struct LevelLoaded(pub Entity);

#[derive(Debug, Default)]
pub struct LevelChangeFsm {
    belt_empty: bool,
    piece_placed: bool,
    bag_offscreen: bool,
}

fn transition_handler(
    mut belt_empty_events: EventReader<BeltEmptyEvent>,
    mut piece_placements: EventReader<PiecePlaced>,
    mut bag_offscreen: EventReader<TweenCompleted>,
    mut level_finished: EventWriter<LevelFinished>,
    mut level_fsm: Local<LevelChangeFsm>,
) {
    if belt_empty_events.iter().count() > 0 {
        level_fsm.belt_empty = true;
    }
    if !level_fsm.belt_empty {
        return;
    }

    if piece_placements.iter().count() > 0 {
        level_fsm.piece_placed = true;
    }
    if !level_fsm.piece_placed {
        return;
    }

    for TweenCompleted { user_data, .. } in bag_offscreen.iter() {
        if *user_data & (AnimationEvent::BAG | AnimationEvent::OFFSCREEN).bits() != 0 {
            level_fsm.bag_offscreen = true;
        }
    }
    if !level_fsm.bag_offscreen {
        return;
    }

    level_finished.send(LevelFinished);
    *level_fsm = default();
}

fn level_unload_handler(
    mut commands: Commands,
    mut gb9000: ResMut<GroceryBagger9000>,
    mut level_finished: EventReader<LevelFinished>,
) {
    if level_finished.iter().count() > 0 {
        if let Some(initialized) = gb9000.level_root {
            commands.entity(initialized).despawn_recursive();
            gb9000.level_root = None;
        }
        gb9000.state = LevelEnded;
    }
}
