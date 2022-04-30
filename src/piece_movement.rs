use std::ops::Deref;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    bags::{BAG_BOUNDARY_COLLIDER_GROUP, BAG_COLLIDER_GROUP},
    nomino_consts::ROTATION_90,
    nominos::*,
    window_management::MainCamera,
    window_utils::compute_cursor_position,
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
    collider: ColliderHandle,
}

impl Deref for SelectedPiece {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

#[derive(Component)]
struct PieceSelectedMarker;

fn piece_selection_handler(
    mut commands: Commands,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    selected_shape: Query<
        (&ColliderPositionComponent, &ColliderShapeComponent),
        With<PieceSelectedMarker>,
    >,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut selected_piece: ResMut<PieceSelection>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(piece) = &**selected_piece {
        let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
        let (pos, shape) = selected_shape.get(**piece).unwrap();

        let intersects_with_bag = query_pipeline.intersection_with_shape(
            &collider_set,
            pos,
            &***shape,
            BAG_COLLIDER_GROUP,
            None,
        );
        let overlaps_boundary = query_pipeline.intersection_with_shape(
            &collider_set,
            pos,
            &***shape,
            BAG_BOUNDARY_COLLIDER_GROUP,
            None,
        );
        let overlaps_other_shapes = query_pipeline.intersection_with_shape(
            &collider_set,
            pos,
            &***shape,
            NOMINO_COLLIDER_GROUP,
            Some(&(|handle| handle != piece.collider)),
        );

        if intersects_with_bag.is_some()
            && overlaps_boundary.is_none()
            && overlaps_other_shapes.is_none()
        {
            commands.entity(**piece).remove::<PieceSelectedMarker>();
            *selected_piece = default();
        }

        return;
    }

    if let Some(cursor_position) = compute_cursor_position(windows, camera) {
        let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
        query_pipeline.intersections_with_point(
            &collider_set,
            &cursor_position.extend(0.).into(),
            NOMINO_COLLIDER_GROUP,
            None,
            |handle| {
                let id = handle.entity();

                commands.entity(id).insert(PieceSelectedMarker);
                *selected_piece = PieceSelection(Some(SelectedPiece {
                    id,
                    collider: handle,
                }));

                false
            },
        );
    }
}

fn piece_rotation_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    mut pieces: Query<(&mut Transform, &mut ColliderPositionComponent), With<PieceSelectedMarker>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) &&
    let Ok((mut piece, mut phys_piece)) = pieces.get_single_mut()
    {
        piece.rotation *= *ROTATION_90;
        *phys_piece = (piece.translation, piece.rotation).into();
    }
}

fn selected_piece_mover(
    mut position: Query<
        (&mut Transform, &mut ColliderPositionComponent),
        With<PieceSelectedMarker>,
    >,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Some(cursor_position) = compute_cursor_position(windows, camera_query) &&
    let Ok((mut position, mut physics_position)) = position.get_single_mut()
    {
        position.translation = cursor_position.round().extend(0.);

        *physics_position = (position.translation, position.rotation).into();
    }
}
