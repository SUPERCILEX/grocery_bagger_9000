use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tweening::TweeningPlugin;

use crate::{
    animations::AnimationPlugin, bags::BagsPlugin, conveyor_belt::ConveyorBeltPlugin,
    levels::LevelsPlugin, nominos::PiecesPlugin,
};

pub struct GroceryBagger9000Plugin;

impl Plugin for GroceryBagger9000Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelsPlugin);
        app.add_plugin(PiecesPlugin);
        app.add_plugin(BagsPlugin);
        app.add_plugin(ConveyorBeltPlugin);
        app.add_plugin(AnimationPlugin);

        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugin(TweeningPlugin);
    }
}
