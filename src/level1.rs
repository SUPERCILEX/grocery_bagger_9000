use bevy::prelude::*;

use crate::{
    bags::BagSpawner,
    conveyor_belt,
    events::PiecePlaced,
    levels::CurrentLevel,
    markers::Selectable,
    nomino_consts::DEG_90,
    nominos::{NominoSpawner, TetrominoL, TetrominoSquare},
    window_management::MainCamera,
    window_utils::get_dips_window,
};

const LEVEL_COLOR: Color = Color::ORANGE;

pub struct Level1Plugin;

impl Plugin for Level1Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_level);
    }
}

#[derive(Deref)]
struct Level1Initialized {
    root: Entity,
}

fn init_level(
    mut commands: Commands,
    current: Res<CurrentLevel>,
    mut initialization: Local<Option<Level1Initialized>>,
    mut placed_pieces: EventWriter<PiecePlaced>,
    windows: Res<Windows>,
    projection_2d: Query<&OrthographicProjection, With<MainCamera>>,
) {
    if current.level >= 1 {
        if let Some(initialized) = &*initialization {
            commands.entity(**initialized).despawn_recursive();
            *initialization = None;
        }
        return;
    } else if initialization.is_some() {
        return;
    }

    let root = commands
        .spawn_bundle(TransformBundle::default())
        .with_children(|parent| {
            let window = get_dips_window(windows.get_primary().unwrap(), projection_2d.single());

            // TODO keep these and the pieces' coordinates up-to-date
            let (bag_position, bag_id) = parent.spawn_bag::<1>(Color::default(), &window)[0];

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

            let pieces = [
                parent
                    .spawn_nomino(
                        bag_position,
                        TetrominoSquare::default(),
                        LEVEL_COLOR,
                        Transform::from_xyz(0., 0., 0.),
                    )
                    .id(),
                parent
                    .spawn_nomino(
                        bag_position,
                        TetrominoSquare::default(),
                        LEVEL_COLOR,
                        Transform::from_xyz(2., 0., 0.),
                    )
                    .id(),
                parent
                    .spawn_nomino(
                        bag_position,
                        TetrominoSquare::default(),
                        LEVEL_COLOR,
                        Transform::from_xyz(4., 0., 0.),
                    )
                    .id(),
                parent
                    .spawn_nomino(
                        bag_position,
                        TetrominoL::default(),
                        LEVEL_COLOR,
                        Transform::from_xyz(1., 3., 0.).with_rotation(*DEG_90),
                    )
                    .id(),
                parent
                    .spawn_nomino(
                        bag_position,
                        TetrominoL::default(),
                        LEVEL_COLOR,
                        Transform::from_xyz(2., 2., 0.).with_rotation((*DEG_90).inverse()),
                    )
                    .id(),
                parent
                    .spawn_nomino(
                        bag_position,
                        TetrominoSquare::default(),
                        LEVEL_COLOR,
                        Transform::from_xyz(4., 2., 0.),
                    )
                    .id(),
                parent
                    .spawn_nomino(
                        bag_position,
                        TetrominoSquare::default(),
                        LEVEL_COLOR,
                        Transform::from_xyz(0., 4., 0.),
                    )
                    .id(),
                parent
                    .spawn_nomino(
                        bag_position,
                        TetrominoL::default(),
                        LEVEL_COLOR,
                        Transform::from_xyz(4., 4., 0.).with_rotation((*DEG_90).inverse()),
                    )
                    .id(),
            ];

            for piece in pieces {
                placed_pieces.send(PiecePlaced { piece, bag: bag_id })
            }
        })
        .id();
    *initialization = Some(Level1Initialized { root });
}
