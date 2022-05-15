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

#[derive(Debug, Default, Eq, PartialEq)]
pub enum LevelChangeFsm {
    #[default]
    Ready,
    BeltEmpty,
    PiecePlaced,
}

fn transition_handler(
    mut belt_empty_events: EventReader<BeltEmptyEvent>,
    mut piece_placements: EventReader<PiecePlaced>,
    mut bag_offscreen: EventReader<TweenCompleted>,
    mut level_finished: EventWriter<LevelFinished>,
    mut level_fsm: Local<LevelChangeFsm>,
) {
    let belt_empty = belt_empty_events.iter().count() > 0;
    let piece_placed = piece_placements.iter().count() > 0;
    let bag_offscreen = bag_offscreen
        .iter()
        .filter(|t| t.user_data & (AnimationEvent::BAG | AnimationEvent::OFFSCREEN).bits() != 0)
        .count()
        > 0;

    match *level_fsm {
        LevelChangeFsm::Ready => {
            if belt_empty {
                *level_fsm = LevelChangeFsm::BeltEmpty;
            }
        }
        LevelChangeFsm::BeltEmpty => {
            if piece_placed {
                *level_fsm = LevelChangeFsm::PiecePlaced;
            }
        }
        LevelChangeFsm::PiecePlaced => {
            if bag_offscreen {
                level_finished.send(LevelFinished);
                *level_fsm = default();
            }
        }
    }
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
