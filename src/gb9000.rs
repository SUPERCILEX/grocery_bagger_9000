use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{levels::LevelsPlugin, piece_movement::PieceMovementPlugin};

pub struct GroceryBagger9000Plugin;

impl Plugin for GroceryBagger9000Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PieceMovementPlugin);
        app.add_plugin(LevelsPlugin);
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}
