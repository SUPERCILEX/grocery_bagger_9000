use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    nomino_consts::ROTATION_90,
    nominos::*,
    window_management::MainCamera,
    window_utils::{compute_cursor_position, window_to_world_coords},
};

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PieceSelection>();
        app.add_system(piece_selection_handler);
        app.add_system(piece_rotation_handler);
        app.add_system(selected_piece_mover.before(piece_selection_handler));
    }
}

#[derive(Deref, DerefMut, Default)]
struct PieceSelection(Option<SelectedPiece>);

struct SelectedPiece {
    id: Entity,
    offset: Vec2,
}

#[derive(Component)]
struct PieceSelectedMarker;

fn piece_selection_handler(
    mut commands: Commands,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    pieces: Query<&Transform, With<NominoMarker>>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut selected_piece: ResMut<PieceSelection>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if selected_piece.is_some() {
            commands
                .entity((*selected_piece).as_ref().unwrap().id)
                .remove::<PieceSelectedMarker>();
            *selected_piece = default();
            return;
        }

        if let Some(cursor_position) = compute_cursor_position(windows, camera) {
            let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
            query_pipeline.intersections_with_point(
                &collider_set,
                &cursor_position.extend(0.).into(),
                InteractionGroups::all(),
                None,
                |handle| {
                    let id = handle.entity();

                    let transform = pieces.get(id).unwrap();
                    let offset = (transform.rotation.inverse()
                        * (cursor_position - transform.translation.truncate()).extend(0.))
                    .truncate();

                    commands.entity(id).insert(PieceSelectedMarker);
                    *selected_piece = PieceSelection(Some(SelectedPiece { id, offset }));

                    false
                },
            );
        }
    }
}

fn piece_rotation_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut pieces: Query<(&mut Transform, &mut ColliderPositionComponent), With<PieceSelectedMarker>>,
) {
    if mouse_button_input.just_released(MouseButton::Right) &&
    let Ok((mut piece, mut phys_piece)) = pieces.get_single_mut() &&
    let Some(cursor_position) = compute_cursor_position(windows, camera)
    {
        piece.rotate_around(
            cursor_position.extend(0.),
            *ROTATION_90,
        );
        *phys_piece = (piece.translation, piece.rotation).into();
    }
}

fn selected_piece_mover(
    selected_piece: Res<PieceSelection>,
    mut mouse_movements: EventReader<CursorMoved>,
    mut position: Query<
        (&mut Transform, &mut ColliderPositionComponent),
        With<PieceSelectedMarker>,
    >,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Some(selected_piece) = (*selected_piece).as_ref() {
        let (camera, camera_transform) = camera.single();
        let (mut position, mut physics_position) = position.single_mut();
        for e in mouse_movements.iter() {
            position.translation = (window_to_world_coords(
                e.position,
                windows.get(e.id).unwrap(),
                camera,
                camera_transform,
            ) - (position.rotation * selected_piece.offset.extend(0.))
                .truncate())
            .round()
            .extend(0.);
            *physics_position = (position.translation, position.rotation).into();
        }
    }
}
