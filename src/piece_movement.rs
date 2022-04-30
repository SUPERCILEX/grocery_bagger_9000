use std::ops::Deref;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    bags::BAG_COLLIDER_GROUP, nomino_consts::ROTATION_90, nominos::*,
    window_management::MainCamera, window_utils::compute_cursor_position,
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
    pieces: Query<&Transform, With<NominoMarker>>,
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
        // TODO check for
        //  1. collision with bag
        //  2. NOT collision with bag bounds
        //  3. NOT collision with any other piece
        let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
        let (pos, shape) = selected_shape.get(**piece).unwrap();

        query_pipeline.intersections_with_shape(
            &collider_set,
            pos,
            &***shape,
            BAG_COLLIDER_GROUP,
            None,
            |handle| {
                dbg!(handle);
                true
            },
        );

        commands.entity(**piece).remove::<PieceSelectedMarker>();
        *selected_piece = default();
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

                let transform = pieces.get(id).unwrap();
                let _offset = (transform.rotation.inverse()
                    * (cursor_position - transform.translation.truncate()).extend(0.))
                .truncate();

                commands.entity(id).insert(PieceSelectedMarker);
                *selected_piece = PieceSelection(Some(SelectedPiece { id }));

                false
            },
        );
    }
}

fn piece_rotation_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    _windows: Res<Windows>,
    _camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
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
        let Ok((mut position, mut physics_position)) = position.get_single_mut(){
        position.translation = cursor_position.round().extend(0.);

        *physics_position = (position.translation, position.rotation).into();
    }
}
