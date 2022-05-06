use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    colors::NominoColor,
    conveyor_belt,
    events::PiecePlaced,
    levels::CurrentLevel,
    markers::Selectable,
    nomino_consts::DEG_90,
    nominos::{NominoSpawner, TetrominoL, TetrominoSquare},
    window_management::MainCamera,
    window_utils::get_dips_window,
};

const LEVEL_COLOR: NominoColor = NominoColor::Orange;

pub struct Level1Plugin;

impl Plugin for Level1Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut current: ResMut<CurrentLevel>,
    mut level_initialized: EventWriter<LevelLoaded>,
    mut placed_pieces: EventWriter<PiecePlaced>,
    dips_window: Res<DipsWindow>,
) {
    if current.level != 0 || current.root.is_some() {
        return;
    }

    let root = commands
        .spawn_bundle(TransformBundle::default())
        .with_children(|parent| {
            // TODO keep these and the pieces' coordinates up-to-date
            let (bag_position, bag_id) = parent.spawn_bag::<1>(Color::default(), &dips_window)[0];

            // TODO: let the conveyor belt do this part for us
            let l_position = Transform::from_xyz(
                window.width - conveyor_belt::LENGTH,
                window.height - conveyor_belt::HEIGHT,
                0.,
            );
            parent
                .spawn_nomino(
                    l_position,
                    TetrominoL::default(),
                    LEVEL_COLOR,
                    Transform::from_translation(Vec3::ZERO),
                )
                .insert(Selectable);

            macro_rules! spawn {
                ($nomino:expr, $transform:expr) => {{
                    parent
                        .spawn_nomino(bag_position, $nomino, LEVEL_COLOR, $transform)
                        .id()
                }};
            }

            let pieces = [
                spawn!(TetrominoSquare, Transform::from_xyz(0., 0., 0.)),
                spawn!(TetrominoSquare, Transform::from_xyz(2., 0., 0.)),
                spawn!(TetrominoSquare, Transform::from_xyz(4., 0., 0.)),
                spawn!(
                    TetrominoL,
                    Transform::from_xyz(1., 3., 0.).with_rotation(*DEG_90)
                ),
                spawn!(
                    TetrominoL,
                    Transform::from_xyz(2., 2., 0.).with_rotation((*DEG_90).inverse())
                ),
                spawn!(TetrominoSquare, Transform::from_xyz(4., 2., 0.)),
                spawn!(TetrominoSquare, Transform::from_xyz(0., 4., 0.)),
                spawn!(
                    TetrominoL,
                    Transform::from_xyz(4., 4., 0.).with_rotation((*DEG_90).inverse())
                ),
            ];

            for piece in pieces {
                placed_pieces.send(PiecePlaced { piece, bag: bag_id })
            }
        })
        .id();
    current.root = Some(root);
    level_initialized.send(LevelLoaded(root));
}
