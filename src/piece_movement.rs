use std::ops::Deref;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    bags::{BAG_BOUNDARY_COLLIDER_GROUP, BAG_COLLIDER_GROUP},
    events::PiecePlaced,
    markers::Selectable,
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
}

impl Deref for SelectedPiece {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

fn piece_selection_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_piece: ResMut<PieceSelection>,
    mut placed_events: EventWriter<PiecePlaced>,
    selectables: Query<(), With<Selectable>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    rapier_context: Res<RapierContext>,
    selected_shape: Query<(&Transform, &Collider)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(piece) = &**selected_piece {
        let (transform, collider) = selected_shape.get(**piece).unwrap();

        let intersects_with_bag = rapier_context.intersection_with_shape(
            transform.translation,
            transform.rotation,
            collider,
            BAG_COLLIDER_GROUP.into(),
            None,
        );

        if let Some(bag) = intersects_with_bag
            && !straddles_bag_or_overlaps_pieces(&rapier_context, *transform, collider, **piece) {
            let mut piece_commands = commands.entity(**piece);
            piece_commands.remove::<Selectable>();
            placed_events.send(PiecePlaced {
                piece: **piece,
                bag,
            });

            *selected_piece = default();
        }

        return;
    }

    if let Some(cursor_position) = compute_cursor_position(windows, camera) {
        rapier_context.intersections_with_point(
            cursor_position.extend(0.),
            NOMINO_COLLIDER_GROUP.into(),
            Some(&(|entity| selectables.contains(entity))),
            |id| {
                *selected_piece = PieceSelection(Some(SelectedPiece { id }));
                false
            },
        );
    }
}

fn piece_rotation_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    selected_piece: Res<PieceSelection>,
    mut pieces: Query<&mut Transform>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        if let Some(piece) = &**selected_piece {
            pieces.get_mut(**piece).unwrap().rotation *= *DEG_90;
        }
    }
}

fn selected_piece_mover(
    selected_piece: Res<PieceSelection>,
    mut pieces: Query<(&mut Transform, &Collider)>,
    rapier_context: Res<RapierContext>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Some(piece) = &**selected_piece &&
    let Some(cursor_position) = compute_cursor_position(windows, camera_query)
    {
        let (mut piece_transform, collider) = pieces.get_mut(**piece).unwrap();
        let snapped_cursor_position = cursor_position.round().extend(0.);

        let would_move_over_invalid_position = straddles_bag_or_overlaps_pieces(
            &rapier_context,
            Transform::from_translation(snapped_cursor_position).with_rotation(piece_transform.rotation),
            collider,
            **piece,
        );

        if would_move_over_invalid_position {
            return;
        }

        piece_transform.translation = snapped_cursor_position;
    }
}

fn straddles_bag_or_overlaps_pieces(
    rapier_context: &Res<RapierContext>,
    transform: Transform,
    collider: &Collider,
    self_id: Entity,
) -> bool {
    rapier_context
        .intersection_with_shape(
            transform.translation,
            transform.rotation,
            collider,
            BAG_BOUNDARY_COLLIDER_GROUP.into(),
            None,
        )
        .or_else(|| {
            rapier_context.intersection_with_shape(
                transform.translation,
                transform.rotation,
                collider,
                NOMINO_COLLIDER_GROUP.into(),
                Some(&(|entity| entity != self_id)),
            )
        })
        .is_some()
}
