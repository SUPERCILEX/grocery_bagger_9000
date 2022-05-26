use bevy::prelude::*;

use crate::{
    bags::{compute_bag_coordinates, BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::{tutorials::spawn_text_tutorial, LevelMarker},
    nominos::{Nomino, NominoSpawner, PiecePlaced, DEG_90},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Green;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    placed_pieces: EventWriter<PiecePlaced>,
    asset_server: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    spawn_bag(&mut commands, &dips_window, placed_pieces);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "Items sticking out of the bag\nare worth fewer points…",
    );
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([Piece {
            nomino: Nomino::TetrominoStraight,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        }])),
    );
}

fn spawn_bag(
    commands: &mut Commands,
    dips_window: &DipsWindow,
    mut placed_pieces: EventWriter<PiecePlaced>,
) {
    let bag = commands.spawn_bag(dips_window, [BAG_SIZE_SMALL])[0];

    // TODO use local coordinates after https://github.com/dimforge/bevy_rapier/issues/172
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelMarker)
        .with_children(|parent| {
            let origin = Transform::from_translation(
                compute_bag_coordinates(dips_window, [BAG_SIZE_SMALL])[0] - BAG_SIZE_SMALL.origin(),
            );
            macro_rules! spawn {
                ($nomino:expr, $transform:expr) => {{
                    parent
                        .spawn_nomino_into_bag(origin, $nomino, LEVEL_COLOR, $transform)
                        .id()
                }};
            }

            let pieces = [
                spawn!(
                    Nomino::TetrominoL,
                    Transform::from_xyz(1., 0., 0.).with_rotation(DEG_90.inverse())
                ),
                spawn!(Nomino::TetrominoL, Transform::from_xyz(0., 2., 0.)),
            ];

            for piece in pieces {
                placed_pieces.send(PiecePlaced { piece, bag });
            }
        });
}
