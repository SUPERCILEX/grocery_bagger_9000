use crate::{
    animations::AnimationEvent,
    conveyor_belt::movement::BeltEmptyEvent,
    levels::{
        init::LevelInitLabel,
        transitions::GameState::{LevelEnded, Playing},
    },
};
use bevy::prelude::*;
use bevy_tweening::TweenCompleted;


pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>();

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

#[derive(Debug, Default, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Playing,
    LevelEnded,
}

#[derive(Debug, Default)]
pub struct CurrentLevel {
    pub level: u16,
    pub root: Option<Entity>,
    pub state: GameState,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct LevelTransitionLabel;

pub struct LevelFinishedEvent;

#[derive(Debug, Default)]
pub struct LevelChangeFSM {
    belt_empty: bool,
    bag_offscreen: bool,
}

fn transition_handler(
    mut belt_empty_events: EventReader<BeltEmptyEvent>,
    mut bag_offscreen: EventReader<TweenCompleted>,
    mut level_finished: EventWriter<LevelFinishedEvent>,
    mut level_fsm: Local<LevelChangeFSM>,
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
    mut current_level: ResMut<CurrentLevel>,
    mut level_finished: EventReader<LevelFinishedEvent>,
) {
    if level_finished.iter().count() > 0 {
        if let Some(initialized) = current_level.root {
            commands.entity(initialized).despawn_recursive();
            current_level.root = None;
        }
        current_level.state = LevelEnded;
    }
}

#[derive(Deref)]
pub struct LevelLoaded(pub Entity);

fn level_change_handler(
    mut commands: Commands,
    mut current: ResMut<CurrentLevel>,
    mut prev_level: Local<u16>,
) {
    if *prev_level != current.level {
        *prev_level = current.level;
        if let Some(initialized) = current.root {
            commands.entity(initialized).despawn_recursive();
            current.root = None;
            current.state = Playing;
        }
    }
}
