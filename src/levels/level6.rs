use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltInstance, RandomPiecesConveyorBelt},
    levels::{CurrentLevel, LevelLoaded},
    nominos::TETROMINOS,
    window_management::DipsWindow,
};

const NUM_PIECES: usize = 18;

pub struct Level6Plugin;

impl Plugin for Level6Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut current: ResMut<CurrentLevel>,
    mut level_initialized: EventWriter<LevelLoaded>,
    mut conveyor_belt: ResMut<ConveyorBeltInstance>,
    dips_window: Res<DipsWindow>,
) {
    if current.level != 5 || current.root.is_some() {
        return;
    }

    let root = commands
        .spawn_bundle(TransformBundle::default())
        .with_children(|parent| {
            parent.spawn_bag::<2>(&dips_window);

            **conveyor_belt = Some(Box::new(RandomPiecesConveyorBelt::new(
                NUM_PIECES,
                TETROMINOS,
                [NominoColor::Blue, NominoColor::Green],
            )));
        })
        .id();
    current.root = Some(root);
    level_initialized.send(LevelLoaded(root));
}
