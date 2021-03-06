use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_tweening::{AnimationSystem, TweenCompleted};

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
        app.add_stage_before(
            CoreStage::PostUpdate,
            LevelSpawnStage,
            SystemStage::parallel(),
        );

        app.add_event::<LevelStarted>();
        app.add_event::<LevelFinished>();

        app.add_system(level_start_handler.label(LevelTransitionSystems));
        app.add_system(
            level_end_handler
                .label(LevelTransitionSystems)
                .after(AnimationSystem::AnimationUpdate)
                .before(level_start_handler),
        );
        app.add_system(
            level_unload_handler
                .label(LevelTransitionSystems)
                .with_run_criteria(run_if_level_finished)
                .after(level_end_handler),
        );
    }
}

// TODO remove after stageless when we can do a command flush
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct LevelSpawnStage;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct LevelTransitionSystems;

#[derive(Default, Component)]
pub struct LevelMarker;

#[derive(Deref)]
pub struct LevelStarted(u16);

pub struct LevelFinished;

fn level_start_handler(
    gb9000: Res<GroceryBagger9000>,
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
    mut gb9000: ResMut<GroceryBagger9000>,
    mut belt_empty_events: EventReader<BeltEmptyEvent>,
    mut completed_animations: EventReader<TweenCompleted>,
    mut level_started: EventReader<LevelStarted>,
    mut level_finished: EventWriter<LevelFinished>,
    mut level_fsm: Local<LevelChangeFsm>,
) {
    if level_started.iter().count() > 0 {
        *level_fsm = LevelChangeFsm::Ready;
    }

    let belt_empty = belt_empty_events.iter().count() > 0;
    let bag_despawned = completed_animations
        .iter()
        .filter(|t| {
            let flags = (AnimationEvent::BAG | AnimationEvent::DESPAWNABLE).bits();
            t.user_data & flags == flags
        })
        .count()
        > 0;

    match *level_fsm {
        LevelChangeFsm::Ready => {
            if belt_empty {
                *level_fsm = LevelChangeFsm::PiecePlaced;
            }
        }
        LevelChangeFsm::PiecePlaced => {
            if bag_despawned {
                gb9000.state = LevelEnded;
                level_finished.send(LevelFinished);
                *level_fsm = default();
            }
        }
    }
}

fn run_if_level_finished(mut level_finished: EventReader<LevelFinished>) -> ShouldRun {
    if level_finished.iter().count() > 0 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn level_unload_handler(mut commands: Commands, level: Query<Entity, With<LevelMarker>>) {
    for entity in level.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
