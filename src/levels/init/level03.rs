use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::transitions::LevelSpawnStage,
    nominos::{Nomino, PiecePlaced, DEG_180, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Pink;

pub struct Level3Plugin;

impl Plugin for Level3Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(LevelSpawnStage, init_level);
    }
}

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    _: EventWriter<PiecePlaced>,
    _: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, [BAG_SIZE_SMALL]);
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    macro_rules! piece {
        ($nomino:expr, $rotation:expr) => {{
            Piece {
                nomino: $nomino,
                color: LEVEL_COLOR,
                rotation: $rotation,
            }
        }};
    }

    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            piece!(Nomino::TetrominoL, Quat::IDENTITY),
            piece!(Nomino::TetrominoSkew, *DEG_MIRRORED),
            piece!(Nomino::TetrominoL, *DEG_180),
        ])),
    );
}
