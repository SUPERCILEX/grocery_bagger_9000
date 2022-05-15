use bevy::prelude::*;

pub use spawn::RobotMarker;
use timing::RobotTimingPlugin;
pub use timing::{RobotTiming, PLACEMENT_TTL};

mod spawn;
mod timing;

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RobotTimingPlugin);
    }
}
