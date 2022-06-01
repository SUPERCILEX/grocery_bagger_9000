use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::transitions::LevelSpawnStage,
    nominos::{Nomino, NominoSpawner, DEG_180, DEG_90, DEG_MIRRORED},
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
    game_speed: Res<GameSpeed>,
    _: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    spawn_bag(&mut commands, &dips_window, &game_speed);
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
            piece!(Nomino::TetrominoSkew, *DEG_MIRRORED),
            piece!(Nomino::TetrominoL, *DEG_180),
        ])),
    );
}

fn spawn_bag(commands: &mut Commands, dips_window: &DipsWindow, game_speed: &GameSpeed) {
    let bag = commands.spawn_bag(dips_window, game_speed, [BAG_SIZE_SMALL])[0];

    commands.entity(bag).with_children(|parent| {
        let origin = Transform::from_translation(-BAG_SIZE_SMALL.origin());
        macro_rules! spawn {
            ($nomino:expr, $transform:expr) => {{
                parent
                    .spawn_nomino_into_bag(origin, $nomino, LEVEL_COLOR, $transform)
                    .id()
            }};
        }

        spawn!(
            Nomino::TetrominoL,
            Transform::from_xyz(1., 0., 0.).with_rotation(DEG_90.inverse())
        );
    });
}
