use std::time::Duration;

use bevy::prelude::*;

use crate::{
    nominos::{PiecePlaced, PieceSystems},
    robot::spawn::RobotMarker,
};

pub const PLACEMENT_TTL: Duration = Duration::from_secs(5);

pub struct RobotTimingPlugin;

impl Plugin for RobotTimingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(accumulate_left_over_time.after(PieceSystems));
        app.add_system(place_piece.after(PieceSystems));
    }
}

#[derive(Component)]
pub struct RobotTiming {
    ttl: Timer,
}

impl Default for RobotTiming {
    fn default() -> Self {
        Self {
            ttl: Timer::new(PLACEMENT_TTL, false),
        }
    }
}

impl RobotTiming {
    pub fn time_left(&self) -> Duration {
        self.ttl.duration()
    }
}

fn accumulate_left_over_time(
    mut piece_placements: EventReader<PiecePlaced>,
    mut timing: Query<&mut RobotTiming, With<RobotMarker>>,
) {
    if piece_placements.iter().count() == 0 {
        return;
    }

    if let Ok(mut robot) = timing.get_single_mut() {
        let ttl = &mut robot.ttl;
        ttl.set_duration(ttl.duration() - ttl.elapsed() + PLACEMENT_TTL);
        ttl.set_elapsed(Duration::ZERO);
    }
}

fn place_piece(time: Res<Time>, mut timing: Query<&mut RobotTiming, With<RobotMarker>>) {
    for mut robot in timing.iter_mut() {
        robot.ttl.tick(time.delta());
        // TODO
    }
}
