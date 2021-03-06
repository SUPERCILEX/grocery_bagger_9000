use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::{Nomino, NominoColor, NominoSpawner, DEG_90, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Gold;

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
        "For best results, try distributing\nitems among the bags.",
    );
    spawn_belt(&mut commands, &dips_window);
}

fn spawn_bag(commands: &mut Commands, dips_window: &DipsWindow, game_speed: &GameSpeed) {
    let bags = commands.spawn_bag(dips_window, game_speed, &[BAG_SIZE_SMALL, BAG_SIZE_SMALL]);
    let [bag1, bag2, ..] = bags.as_slice() else { unreachable!() };
    let origin = Transform::from_translation(-BAG_SIZE_SMALL.origin());

    commands.entity(*bag1).with_children(|parent| {
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

    commands.entity(*bag2).with_children(|parent| {
        macro_rules! spawn {
            ($nomino:expr, $transform:expr) => {{
                parent
                    .spawn_nomino_into_bag(origin, $nomino, LEVEL_COLOR, $transform)
                    .id()
            }};
        }

        spawn!(Nomino::TetrominoSquare, Transform::from_xyz(0., 0., 0.));
    });
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    macro_rules! piece {
        (mirrored $nomino:expr) => {{
            Piece {
                nomino: $nomino,
                color: LEVEL_COLOR,
                rotation: *DEG_MIRRORED,
            }
        }};

        ($nomino:expr) => {{
            Piece {
                nomino: $nomino,
                color: LEVEL_COLOR,
                rotation: Quat::IDENTITY,
            }
        }};
    }

    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            piece!(mirrored Nomino::TetrominoSkew),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoSquare),
            piece!(Nomino::TetrominoStraight),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoL),
            piece!(Nomino::TetrominoStraight),
        ])),
    );
}
