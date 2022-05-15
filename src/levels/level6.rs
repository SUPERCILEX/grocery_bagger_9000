use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, RandomPiecesConveyorBelt},
    gb9000::GroceryBagger9000,
    levels::{
        init::{level_init_chrome, LevelInitLabel},
        LevelLoaded,
    },
    nominos::TETROMINOS,
    window_management::DipsWindow,
};

const NUM_PIECES: usize = 18;

pub struct Level6Plugin;

impl Plugin for Level6Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level.label(LevelInitLabel));
    }
}

fn init_level(
    mut commands: Commands,
    gb9000: ResMut<GroceryBagger9000>,
    level_loaded: EventWriter<LevelLoaded>,
    dips_window: Res<DipsWindow>,
) {
    level_init_chrome(6, gb9000, level_loaded, || {
        commands
            .spawn_bundle(TransformBundle::default())
            .with_children(|parent| {
                parent.spawn_bag::<2>(&dips_window);

                parent.spawn_belt(Box::new(RandomPiecesConveyorBelt::new(
                    NUM_PIECES,
                    TETROMINOS,
                    [NominoColor::Blue, NominoColor::Green],
                )));
            })
            .id()
    });
}
