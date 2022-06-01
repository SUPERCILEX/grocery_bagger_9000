use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::{Nomino, NominoSpawner, DEG_90},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Gold;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    asset_server: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    spawn_bag(&mut commands, &dips_window, &game_speed);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "Sometimes, leaving holes under\nitems is unavoidable…\nTry rearranging the items for a higher score",
    );
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    macro_rules! piece {
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
            piece!(Nomino::TetrominoStraight),
            piece!(Nomino::TetrominoStraight),
            piece!(Nomino::TetrominoSquare),
            piece!(Nomino::TrominoL),
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
            Nomino::TrominoL,
            Transform::from_xyz(0., 1., 0.).with_rotation(*DEG_90)
        );
        spawn!(Nomino::TetrominoSquare, Transform::from_xyz(0., 2., 0.));
    });
}
