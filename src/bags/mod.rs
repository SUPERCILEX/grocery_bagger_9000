use bevy::prelude::*;

pub use consts::*;
// TODO remove https://github.com/dimforge/bevy_rapier/issues/172
pub use positioning::compute_bag_coordinates;
pub use spawn::{BagMarker, BagSpawner};

use crate::bags::{bag_replacement::BagReplacementPlugin, positioning::BagPositioningPlugin};

mod bag_replacement;
mod consts;
mod positioning;
mod spawn;

pub struct BagsPlugin;

impl Plugin for BagsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BagReplacementPlugin);
        app.add_plugin(BagPositioningPlugin);
    }
}
