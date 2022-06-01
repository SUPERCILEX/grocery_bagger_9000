use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_rapier3d::prelude::*;
use bevy_svg::prelude::SvgPlugin;
use bevy_tweening::TweeningPlugin;

use crate::{
    animations::AnimationPlugin, bags::BagsPlugin, conveyor_belt::ConveyorBeltPlugin,
    levels::LevelsPlugin, nominos::PiecesPlugin, robot::RobotPlugin, ui::UiPlugin,
};

pub struct GroceryBagger9000Plugin;

impl Plugin for GroceryBagger9000Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GroceryBagger9000>();

        app.add_plugin(LevelsPlugin);
        app.add_plugin(PiecesPlugin);
        app.add_plugin(BagsPlugin);
        app.add_plugin(ConveyorBeltPlugin);
        app.add_plugin(AnimationPlugin);
        app.add_plugin(UiPlugin);
        app.add_plugin(RobotPlugin);
        #[cfg(not(debug_assertions))]
        app.add_plugin(crate::analytics::AnalyticsPlugin);

        app.add_plugin(ShapePlugin);
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.insert_resource(RapierConfiguration {
            physics_pipeline_active: false,
            ..default()
        });
        app.add_plugin(TweeningPlugin);
        app.add_plugin(SvgPlugin);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Playing,
    LevelEnded,
}

#[derive(Debug)]
pub struct GroceryBagger9000 {
    pub state: GameState,
    pub current_level: u16,
}

impl Default for GroceryBagger9000 {
    fn default() -> Self {
        Self {
            state: default(),
            current_level: 1,
        }
    }
}
