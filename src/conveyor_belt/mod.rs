use bevy::prelude::*;

pub use consts::*;
pub use data::*;

use crate::conveyor_belt::movement::ConveyorBeltMovementPlugin;

mod consts;
mod data;
mod movement;

pub struct ConveyorBeltPlugin;

impl Plugin for ConveyorBeltPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConveyorBeltInstance>();
        app.init_resource::<ConveyorBeltOptions>();

        app.add_plugin(ConveyorBeltMovementPlugin);
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
