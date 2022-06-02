use bevy::prelude::*;

use bag_replacement::BagReplacementPlugin;
pub use bag_replacement::{BagFilled, BagReplacementDetectionSystems, BagReplacementSystems};
pub use bag_size::BagSize;
pub use consts::*;
use positioning::BagPositioningPlugin;
pub use spawn::{BagContainerSpawner, BagMarker, BagSpawner};

mod bag_replacement;
mod bag_size;
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
