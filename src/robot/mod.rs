use bevy::prelude::*;

pub use spawn::RobotMarker;
use timing::RobotTimingPlugin;
pub use timing::{RobotTargetMarker, RobotTiming};

mod spawn;
mod timing;

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RobotTimingPlugin);
    }
}
