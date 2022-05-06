use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    bag_replacement::BagReplacementPlugin, conveyor_belt_movement::ConveyorBeltMovementPlugin,
    levels::LevelsPlugin, piece_movement::PieceMovementPlugin, scoring::ScoringPlugin,
};

pub struct GroceryBagger9000Plugin;

impl Plugin for GroceryBagger9000Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelsPlugin);
        app.add_plugin(PieceMovementPlugin);
        app.add_plugin(BagReplacementPlugin);
        app.add_plugin(ConveyorBeltMovementPlugin);
        app.add_plugin(ScoringPlugin);

        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}
