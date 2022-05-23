use bevy::prelude::*;

use crate::{
    bags::{compute_bag_coordinates, BagContainerSpawner, BAG_ORIGIN},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::{transitions::LevelSpawnStage, LevelMarker, LevelStarted},
    nominos::{Nomino, NominoSpawner, PiecePlaced, DEG_90, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Green;

pub struct Level2Plugin;

impl Plugin for Level2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(LevelSpawnStage, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut level_started: EventReader<LevelStarted>,
    mut placed_pieces: EventWriter<PiecePlaced>,
    dips_window: Res<DipsWindow>,
) {
    if !level_started.iter().last().map(|l| **l).contains(&1) {
        return;
    }

    commands.spawn_belt(Box::new(PresetPiecesConveyorBelt::new([
        Piece {
            nomino: Nomino::TetrominoStraight,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
        Piece {
            nomino: Nomino::TetrominoT,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        },
    ])));

    let bag = commands.spawn_bag::<1>(&dips_window)[0];

    // TODO use local coordinates after https://github.com/dimforge/bevy_rapier/issues/172
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelMarker)
        .with_children(|parent| {
            let origin = Transform::from_translation(
                compute_bag_coordinates(&dips_window, 1)[0] - BAG_ORIGIN,
            );
            macro_rules! spawn {
                ($nomino:expr, $transform:expr) => {{
                    parent
                        .spawn_nomino_into_bag(origin, $nomino, LEVEL_COLOR, $transform)
                        .id()
                }};
            }

            let pieces = [
                spawn!(Nomino::TetrominoL, Transform::from_xyz(0., 1., 0.)),
                spawn!(
                    Nomino::TetrominoT,
                    Transform::from_xyz(3., 0., 0.).with_rotation(DEG_90.inverse())
                ),
                spawn!(Nomino::TetrominoStraight, Transform::from_xyz(5., 2., 0.)),
                spawn!(Nomino::TetrominoSquare, Transform::from_xyz(1., 1., 0.)),
                spawn!(
                    Nomino::TetrominoL,
                    Transform::from_xyz(1., 3., 0.).with_rotation(DEG_90.inverse())
                ),
                spawn!(Nomino::TetrominoSquare, Transform::from_xyz(0., 4., 0.)),
                spawn!(
                    Nomino::TetrominoSkew,
                    Transform::from_xyz(4., 2., 0.).with_rotation(*DEG_MIRRORED)
                ),
                spawn!(Nomino::TetrominoSkew, Transform::from_xyz(4., 4., 0.)),
            ];

            for piece in pieces {
                placed_pieces.send(PiecePlaced { piece, bag })
            }
        });
}
