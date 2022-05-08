use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltInstance, Piece, PresetPiecesConveyorBelt},
    levels::{CurrentLevel, LevelLoaded},
    nominos::{Nomino, NominoSpawner, PiecePlaced, DEG_90, DEG_MIRRORED},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Green;

pub struct Level2Plugin;

impl Plugin for Level2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut current: ResMut<CurrentLevel>,
    mut level_initialized: EventWriter<LevelLoaded>,
    mut conveyor_belt: ResMut<ConveyorBeltInstance>,
    mut placed_pieces: EventWriter<PiecePlaced>,
    dips_window: Res<DipsWindow>,
) {
    if current.level != 1 || current.root.is_some() {
        return;
    }

    let root = commands
        .spawn_bundle(TransformBundle::default())
        .with_children(|parent| {
            // TODO keep these and the pieces' coordinates up-to-date
            let (bag_position, bag_id) = parent.spawn_bag::<1>(Color::default(), &dips_window)[0];

            **conveyor_belt = Some(Box::new(PresetPiecesConveyorBelt::new([
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

            macro_rules! spawn {
                ($nomino:expr, $transform:expr) => {{
                    parent
                        .spawn_nomino(bag_position, $nomino, LEVEL_COLOR, $transform)
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
                placed_pieces.send(PiecePlaced { piece, bag: bag_id })
            }
        })
        .id();
    current.root = Some(root);
    level_initialized.send(LevelLoaded(root));
}
