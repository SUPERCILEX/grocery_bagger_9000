use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    colors::NominoColor,
    conveyor_belt::{
        ConveyorBeltInstance, Piece, PresetPiecesConveyorBelt, RandomPiecesConveyorBelt,
    },
    levels::{CurrentLevel, LevelLoaded},
    nominos::{Nomino, DEG_180, DEG_MIRRORED, TETROMINOS},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Pink;

pub struct Level6Plugin;

impl Plugin for Level6Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level);
    }
}

const NUM_PIECES: usize = 18;

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

//                 Piece {
//                     nomino: Nomino::TetrominoL,
//                     color: LEVEL_COLOR,
//                     rotation: *DEG_MIRRORED * *DEG_180,
//                 },
