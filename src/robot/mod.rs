use bevy::prelude::*;

pub use spawn::{RobotMarker, RobotSpawner};
use timing::RobotTimingPlugin;
pub use timing::{RobotTargetMarker, RobotTiming};

mod spawn;
mod timing;

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RobotTimingPlugin);

        init_robot(app);
    }
}

pub struct RobotOptions {
    pub enabled: bool,
}

impl Default for RobotOptions {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[cfg(feature = "debug")]
fn init_robot(app: &mut App) {
    app.init_resource::<RobotOptions>();
}

#[cfg(not(feature = "debug"))]
fn init_robot(app: &mut App) {
    use rand::{thread_rng, Rng};

    let mut options = RobotOptions::default();
    options.enabled = thread_rng().gen();
    app.insert_resource(options);
}
