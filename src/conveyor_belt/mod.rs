use bevy::prelude::*;

pub use consts::{HEIGHT, MAX_NUM_PIECES};
pub use data::*;
use movement::ConveyorBeltMovementPlugin;
pub use movement::{BeltEmptyEvent, BeltMovementSystems};
use positioning::ConveyorBeltPositioningPlugin;
pub use spawn::{BoxedConveyorBelt, ConveyorBeltSpawner};

mod consts;
mod data;
mod movement;
mod positioning;
mod spawn;

pub struct ConveyorBeltPlugin;

impl Plugin for ConveyorBeltPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConveyorBeltOptions>();

        app.add_plugin(ConveyorBeltMovementPlugin);
        app.add_plugin(ConveyorBeltPositioningPlugin);
    }
}

pub struct ConveyorBeltOptions {
    pub num_pieces_selectable: u8,
}

impl Default for ConveyorBeltOptions {
    fn default() -> Self {
        Self {
            num_pieces_selectable: 3,
        }
    }
}
