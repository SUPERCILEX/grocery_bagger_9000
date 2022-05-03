use std::ops::Deref;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    bags::{BAG_BOUNDARY_COLLIDER_GROUP, BAG_COLLIDER_GROUP},
    events::PiecePlaced,
    nomino_consts::DEG_90,
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

// TODO get rid of this
#[derive(Component)]
struct PieceSelectedMarker;

fn piece_selection_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_piece: ResMut<PieceSelection>,
    mut placed_events: EventWriter<PiecePlaced>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    selected_shape: Query<
        (&ColliderPositionComponent, &ColliderShapeComponent),
        With<PieceSelectedMarker>,
    >,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
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

        if let Some(bag_handle) = intersects_with_bag && !straddles_bag_or_overlaps_pieces(
            &query_pipeline,
            piece.collider,
            &collider_set,
            pos,
            shape,
        ) {
            let mut piece_commands = commands.entity(**piece);
            piece_commands.remove::<PieceSelectedMarker>();
            placed_events.send(PiecePlaced {
                piece: **piece,
                bag: bag_handle.entity(),
            });

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
        piece.rotation *= *DEG_90;
        *phys_piece = (piece.translation, piece.rotation).into();
    }
}

fn selected_piece_mover(
    selected_piece: Res<PieceSelection>,
    mut piece_queries: ParamSet<(
        Query<
            (
                &mut Transform,
                &mut ColliderPositionComponent,
                &ColliderShapeComponent,
            ),
            With<PieceSelectedMarker>,
        >,
        QueryPipelineColliderComponentsQuery,
    )>,
    query_pipeline: Res<QueryPipeline>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Some(selected_piece) = (*selected_piece).as_ref() &&
    let Some(cursor_position) = compute_cursor_position(windows, camera_query)
    {
        let snapped_cursor_position = cursor_position.round().extend(0.);

        {
            let (rotation, collider_shape) = {
                let position_query = piece_queries.p0();
                let (position, _, collider_shape) = position_query.single();
                (position.rotation, (*collider_shape).clone())
            };

            let collider_query = piece_queries.p1();
            let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

            let would_move_over_invalid_position = straddles_bag_or_overlaps_pieces(
                &query_pipeline,
                selected_piece.collider,
                &collider_set,
                &(snapped_cursor_position, rotation).into(),
                &collider_shape,
            );

            if would_move_over_invalid_position {
                return;
            }
        }

        let mut position_query = piece_queries.p0();
        let (mut position, mut physics_position, ..) = position_query.single_mut();

        position.translation = snapped_cursor_position;
        *physics_position = (position.translation, position.rotation).into();
    }
}

fn straddles_bag_or_overlaps_pieces(
    query_pipeline: &Res<QueryPipeline>,
    selected_piece_collider: ColliderHandle,
    collider_set: &QueryPipelineColliderComponentsSet,
    pos: &ColliderPosition,
    shape: &ColliderShape,
) -> bool {
    query_pipeline
        .intersection_with_shape(
            collider_set,
            pos,
            &**shape,
            BAG_BOUNDARY_COLLIDER_GROUP,
            None,
        )
        .or_else(|| {
            query_pipeline.intersection_with_shape(
                collider_set,
                pos,
                &**shape,
                NOMINO_COLLIDER_GROUP,
                Some(&(|handle| handle != selected_piece_collider)),
            )
        })
        .is_some()
}
