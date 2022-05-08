use bevy::prelude::*;

pub use consts::*;
pub use spawn::BagSpawner;

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
