use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    bag_replacement::BagReplacementPlugin, events::EventsPlugin, levels::LevelsPlugin,
    piece_movement::PieceMovementPlugin, scoring::ScoringPlugin,
};

pub struct GroceryBagger9000Plugin;

impl Plugin for GroceryBagger9000Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PieceMovementPlugin);
        app.add_plugin(LevelsPlugin);
        app.add_plugin(EventsPlugin);
        app.add_plugin(BagReplacementPlugin);
        app.add_plugin(ScoringPlugin);
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}
