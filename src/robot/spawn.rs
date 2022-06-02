use bevy::prelude::*;

use crate::{levels::LevelMarker, robot::RobotTiming};

#[derive(Default, Component)]
pub struct RobotMarker;

pub trait RobotSpawner<'w, 's> {
    fn spawn_robot(&mut self);
}

#[derive(Default, Bundle)]
struct RobotBundle {
    #[bundle]
    transforms: TransformBundle,
    level_marker: LevelMarker,
    robot_marker: RobotMarker,
    robot: RobotTiming,
}

impl<'w, 's> RobotSpawner<'w, 's> for Commands<'w, 's> {
    fn spawn_robot(&mut self) {
        self.spawn_and_forget(RobotBundle::default());
    }
}
