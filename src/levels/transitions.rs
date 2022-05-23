use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

use crate::{
    animations::AnimationEvent,
    conveyor_belt::BeltEmptyEvent,
    gb9000::{
        GameState::{LevelEnded, Playing},
        GroceryBagger9000,
    },
};

pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelStarted>();
        app.add_event::<LevelFinished>();

        app.add_system_to_stage(
            CoreStage::Last,
            level_start_handler.label(LevelTransitionLabel),
        );
        app.add_system_to_stage(
            CoreStage::Last,
            level_end_handler.label(LevelTransitionLabel),
        );
        app.add_system_to_stage(
            CoreStage::Last,
            level_unload_handler
                .after(level_end_handler)
                .label(LevelTransitionLabel),
        );
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct LevelTransitionLabel;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct LevelInitLabel;

#[derive(Component)]
pub struct LevelMarker;

#[derive(Deref)]
pub struct LevelStarted(u16);

pub struct LevelFinished;

fn level_start_handler(
    gb9000: ResMut<GroceryBagger9000>,
    mut level_started: EventWriter<LevelStarted>,
    level: Query<(), With<LevelMarker>>,
) {
    if gb9000.state == Playing && level.is_empty() {
        level_started.send(LevelStarted(gb9000.current_level));
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum LevelChangeFsm {
    #[default]
    Ready,
    PiecePlaced,
}

fn level_end_handler(
    mut belt_empty_events: EventReader<BeltEmptyEvent>,
    mut bag_offscreen: EventReader<TweenCompleted>,
    mut level_started: EventReader<LevelStarted>,
    mut level_finished: EventWriter<LevelFinished>,
    mut level_fsm: Local<LevelChangeFsm>,
) {
    if level_started.iter().count() > 0 {
        *level_fsm = LevelChangeFsm::Ready;
    }

    let belt_empty = belt_empty_events.iter().count() > 0;
    let bag_offscreen = bag_offscreen
        .iter()
        .filter(|t| t.user_data & (AnimationEvent::BAG | AnimationEvent::OFFSCREEN).bits() != 0)
        .count()
        > 0;

    match *level_fsm {
        LevelChangeFsm::Ready => {
            if belt_empty {
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
    level: Query<Entity, With<LevelMarker>>,
) {
    if level_finished.iter().count() > 0 {
        for entity in level.iter() {
            commands.entity(entity).despawn_recursive();
        }
        gb9000.state = LevelEnded;
    }
}
