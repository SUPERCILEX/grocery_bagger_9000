use bevy::prelude::*;

use crate::{
    bags::{compute_bag_coordinates, BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::{tutorials::spawn_text_tutorial, LevelMarker},
    nominos::{Nomino, NominoSpawner, PiecePlaced, DEG_90, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR_1: NominoColor = NominoColor::Gold;
const LEVEL_COLOR_2: NominoColor = NominoColor::Blue;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    placed_pieces: EventWriter<PiecePlaced>,
    asset_server: Res<AssetServer>,
) {
    spawn_bag(&mut commands, &dips_window, placed_pieces);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "For best results, group items of the same color into a single bag",
    );
    spawn_belt(&mut commands, &dips_window);
}

fn spawn_bag(
    commands: &mut Commands,
    dips_window: &DipsWindow,
    mut placed_pieces: EventWriter<PiecePlaced>,
) {
    let [bag1, bag2, ..] = commands
        .spawn_bag(dips_window, [BAG_SIZE_SMALL, BAG_SIZE_SMALL])
        .as_slice() else { unreachable!() };
    let [coords1, coords2, ..] =
        compute_bag_coordinates(dips_window, [BAG_SIZE_SMALL, BAG_SIZE_SMALL]).as_slice() else { unreachable!() };
    let (origin1, origin2) = (
        Transform::from_translation(*coords1 - BAG_SIZE_SMALL.origin()),
        Transform::from_translation(*coords2 - BAG_SIZE_SMALL.origin()),
    );

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelMarker)
        .with_children(|parent| {
            macro_rules! spawn {
                ($nomino:expr, $transform:expr) => {{
                    parent
                        .spawn_nomino_into_bag(origin1, $nomino, LEVEL_COLOR_1, $transform)
                        .id()
                }};
            }
            let pieces = [spawn!(
                Nomino::TetrominoL,
                Transform::from_xyz(1., 0., 0.).with_rotation(DEG_90.inverse())
            )];

            for piece in pieces {
                placed_pieces.send(PiecePlaced { piece, bag: *bag1 })
            }
        });

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelMarker)
        .with_children(|parent| {
            macro_rules! spawn {
                ($nomino:expr, $transform:expr) => {{
                    parent
                        .spawn_nomino_into_bag(origin2, $nomino, LEVEL_COLOR_2, $transform)
                        .id()
                }};
            }
            let pieces = [spawn!(
                Nomino::TetrominoSquare,
                Transform::from_xyz(0., 0., 0.)
            )];

            for piece in pieces {
                placed_pieces.send(PiecePlaced { piece, bag: *bag2 })
            }
        });
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([
            Piece {
                nomino: Nomino::TetrominoL,
                color: LEVEL_COLOR_1,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoStraight,
                color: LEVEL_COLOR_2,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoSquare,
                color: LEVEL_COLOR_2,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoSkew,
                color: LEVEL_COLOR_1,
                rotation: *DEG_MIRRORED,
            },
            Piece {
                nomino: Nomino::TetrominoL,
                color: LEVEL_COLOR_2,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoL,
                color: LEVEL_COLOR_1,
                rotation: *DEG_MIRRORED,
            },
            Piece {
                nomino: Nomino::TetrominoL,
                color: LEVEL_COLOR_2,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoSquare,
                color: LEVEL_COLOR_1,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoL,
                color: LEVEL_COLOR_1,
                rotation: Quat::IDENTITY,
            },
            Piece {
                nomino: Nomino::TetrominoStraight,
                color: LEVEL_COLOR_2,
                rotation: Quat::IDENTITY,
            },
        ])),
    );
}
