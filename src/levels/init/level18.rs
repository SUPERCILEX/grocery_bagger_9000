use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::{Nomino, NominoColor, NominoSpawner, DEG_90, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR_1: NominoColor = NominoColor::Gold;
const LEVEL_COLOR_2: NominoColor = NominoColor::Blue;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    asset_server: Res<AssetServer>,
) {
    spawn_bag(&mut commands, &dips_window, &game_speed);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "For best results, group items of\nthe same color into a single bag.",
    );
    spawn_belt(&mut commands, &dips_window);
}

fn spawn_bag(commands: &mut Commands, dips_window: &DipsWindow, game_speed: &GameSpeed) {
    let [bag1, bag2, ..] = commands
        .spawn_bag(dips_window, game_speed, &[BAG_SIZE_SMALL, BAG_SIZE_SMALL])
        .as_slice() else { unreachable!() };
    let origin = Transform::from_translation(-BAG_SIZE_SMALL.origin());

    commands.entity(*bag1).with_children(|parent| {
        macro_rules! spawn {
            ($nomino:expr, $transform:expr) => {{
                parent
                    .spawn_nomino_into_bag(origin, $nomino, LEVEL_COLOR_1, $transform)
                    .id()
            }};
        }

        spawn!(
            Nomino::TetrominoL,
            Transform::from_xyz(1., 0., 0.).with_rotation(DEG_90.inverse())
        );
    });

    commands.entity(*bag2).with_children(|parent| {
        macro_rules! spawn {
            ($nomino:expr, $transform:expr) => {{
                parent
                    .spawn_nomino_into_bag(origin, $nomino, LEVEL_COLOR_2, $transform)
                    .id()
            }};
        }

        spawn!(Nomino::TetrominoSquare, Transform::from_xyz(0., 0., 0.));
    });
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    macro_rules! piece {
        (mirrored $nomino:expr, $color:expr) => {{
            Piece {
                nomino: $nomino,
                color: $color,
                rotation: *DEG_MIRRORED,
            }
        }};

        ($nomino:expr, $color:expr) => {{
            Piece {
                nomino: $nomino,
                color: $color,
                rotation: Quat::IDENTITY,
            }
        }};
    }

    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            piece!(Nomino::TetrominoL, LEVEL_COLOR_1),
            piece!(Nomino::TetrominoStraight, LEVEL_COLOR_2),
            piece!(Nomino::TetrominoSquare, LEVEL_COLOR_2),
            piece!(mirrored Nomino::TetrominoSkew, LEVEL_COLOR_1),
            piece!(Nomino::TetrominoL, LEVEL_COLOR_2),
            piece!(mirrored Nomino::TetrominoL, LEVEL_COLOR_1),
            piece!(Nomino::TetrominoL, LEVEL_COLOR_2),
            piece!(Nomino::TetrominoSquare, LEVEL_COLOR_1),
            piece!(Nomino::TetrominoL, LEVEL_COLOR_1),
            piece!(Nomino::TetrominoStraight, LEVEL_COLOR_2),
        ])),
    );
}
