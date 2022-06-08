use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::{tutorials::spawn_text_tutorial, LevelSpawnStage, LevelStarted},
    nominos::{Nomino, NominoColor, DEG_MIRRORED},
    robot::{RobotOptions, RobotSpawner},
    window_management::DipsWindow,
};

pub struct Level11Plugin;

impl Plugin for Level11Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            LevelSpawnStage,
            show_tutorial.after(super::super::init_levels),
        );
    }
}

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    _: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(&dips_window, &game_speed, &[BAG_SIZE_SMALL]);
    commands.spawn_robot();
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    macro_rules! piece {
        ($nomino:expr, $color:expr) => {{
            Piece {
                nomino: $nomino,
                color: $color,
                rotation: Quat::IDENTITY,
            }
        }};

        ($nomino:expr, Mirrored, $color:expr) => {{
            Piece {
                nomino: $nomino,
                color: $color,
                rotation: *DEG_MIRRORED,
            }
        }};
    }

    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            piece!(Nomino::TetrominoSquare, NominoColor::Gold),
            piece!(Nomino::TetrominoL, Mirrored, NominoColor::Gold),
            piece!(Nomino::TetrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoL, NominoColor::Gold),
            piece!(Nomino::TrominoStraight, NominoColor::Gold),
        ])),
    );
}

fn show_tutorial(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    robot_options: Res<RobotOptions>,
    mut level_started: EventReader<LevelStarted>,
) {
    if let Some(started) = level_started.iter().last() && **started == 11 && robot_options.enabled {
        spawn_text_tutorial(
            &mut commands,
            asset_server,
            "Some levels include a timed robo-bagger.\n\n\
            A new piece will be placed when the outlined\npiece turns solid.\n\n\
            Place any piece to delay the robo-bagger.",
        );
    }
}
