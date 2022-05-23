use bevy::{math::const_vec3, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_tweening::AnimationSystem;

use crate::{
    animations,
    animations::{GameSpeed, Original, UndoableAnimationBundle},
    bags::{BAG_BOUNDARY_COLLIDER_GROUP, BAG_COLLIDER_GROUP, BAG_FLOOR_COLLIDER_GROUP},
    nominos::*,
    window_management::{DipsWindow, MainCamera},
    window_utils::compute_cursor_position,
};

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PiecePlaced>();
        app.add_event::<PiecePickedUp>();

        app.add_system(piece_selection_handler);
        app.add_system(piece_rotation_handler.after(AnimationSystem::AnimationUpdate));
        app.add_system(
            selected_piece_mover
                .before(piece_selection_handler)
                .after(AnimationSystem::AnimationUpdate),
        );
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

pub struct PiecePlaced {
    pub piece: Entity,
    pub bag: Entity,
}

#[derive(Deref)]
pub struct PiecePickedUp(Entity);

const FLOATING_PIECE_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: BAG_FLOOR_COLLIDER_GROUP.memberships | NOMINO_COLLIDER_GROUP.memberships,
    filters: BAG_FLOOR_COLLIDER_GROUP.filters | NOMINO_COLLIDER_GROUP.filters,
};

fn piece_selection_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut picked_up_events: EventWriter<PiecePickedUp>,
    mut placed_events: EventWriter<PiecePlaced>,
    selectables: Query<&Selectable, With<NominoMarker>>,
    game_speed: Res<GameSpeed>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    rapier_context: Res<RapierContext>,
    mut pieces_queries: ParamSet<(
        Query<
            (
                Entity,
                &mut Transform,
                &Collider,
                Option<&Original<Transform>>,
            ),
            (With<NominoMarker>, With<Selected>),
        >,
        Query<(&mut Transform, Option<&Original<Transform>>), With<NominoMarker>>,
    )>,
    #[cfg(feature = "debug")] debug_options: Res<crate::debug::DebugOptions>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    {
        let mut selected_shape = pieces_queries.p0();
        if let Ok((piece, mut transform, collider, original)) = selected_shape.get_single_mut() {
            if let Some(original) = original {
                transform.rotation = original.rotation;
                commands
                    .entity(piece)
                    .remove_bundle::<UndoableAnimationBundle<Transform>>();
            }

            #[cfg(feature = "debug")]
            if debug_options.unrestricted_pieces {
                commands.entity(piece).remove::<Selected>();
                return;
            }

            let intersects_with_bag = rapier_context.intersection_with_shape(
                transform.translation,
                transform.rotation,
                collider,
                BAG_COLLIDER_GROUP.into(),
                None,
            );

            if let Some(bag) = intersects_with_bag
                && !straddles_bag_or_overlaps_pieces(&rapier_context, *transform, collider, piece)
                && !piece_is_floating(&rapier_context, *transform, collider, piece) {
                commands
                    .entity(piece)
                    .remove::<Selectable>()
                    .remove::<Selected>()
                    .insert(animations::piece_placed(*transform, &game_speed));

                placed_events.send(PiecePlaced { piece, bag });
            } else {
                commands
                    .entity(piece)
                    .insert_bundle(animations::error_shake(*transform, &game_speed));
            }

            return;
        }
    }

    if let Some(cursor_position) = compute_cursor_position(windows, camera) {
        let mut failed_selection = None;

        rapier_context.intersections_with_point(
            cursor_position.extend(0.),
            NOMINO_COLLIDER_GROUP.into(),
            None,
            |id| {
                #[allow(unused_mut)]
                let mut selectable = selectables.contains(id);

                #[cfg(feature = "debug")]
                if debug_options.unrestricted_pieces {
                    selectable = true;
                }

                if selectable {
                    picked_up_events.send(PiecePickedUp(id));

                    commands
                        .entity(id)
                        .insert(Selected)
                        .remove_bundle::<UndoableAnimationBundle<Transform>>();
                }
                failed_selection = if selectable { None } else { Some(id) };

                !selectable
            },
        );

        if let Some(failed) = failed_selection {
            let mut piece_positions = pieces_queries.p1();
            let (mut piece_position, original) = piece_positions.get_mut(failed).unwrap();

            if let Some(original) = original {
                piece_position.rotation = original.rotation;
            }

            commands
                .entity(failed)
                .insert_bundle(animations::error_shake(*piece_position, &game_speed));
        }
    }
}

fn piece_rotation_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_piece: Query<
        (Entity, &mut Transform, Option<&Original<Transform>>),
        (With<NominoMarker>, With<Selected>),
    >,
) {
    if !mouse_button_input.just_pressed(MouseButton::Right) {
        return;
    }

    if let Ok((piece, mut transform, original)) = selected_piece.get_single_mut() {
        if let Some(original) = original {
            transform.rotation = original.rotation;
            commands
                .entity(piece)
                .remove_bundle::<UndoableAnimationBundle<Transform>>();
        }

        let rotation = &mut transform.rotation;
        if rotation.x.is_normal() || rotation.y.is_normal() {
            *rotation *= DEG_90.inverse();
        } else {
            *rotation *= *DEG_90;
        }
    }
}

fn selected_piece_mover(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    mut cursor_movements: EventReader<CursorMoved>,
    mut selected_piece: Query<
        (
            Entity,
            &GlobalTransform,
            &mut Transform,
            &Collider,
            Option<&Original<Transform>>,
        ),
        (With<NominoMarker>, With<Selected>),
    >,
    rapier_context: Res<RapierContext>,
) {
    if let Some(moved_event) = cursor_movements.iter().last() &&
    let Ok((piece, global_transform, mut piece_transform, collider, original)) = selected_piece.get_single_mut()
    {
        let cursor_position = moved_event.position * dips_window.scale;

        let snapped_cursor_position = cursor_position
            .round()
            .extend(piece_transform.translation.z);

        if piece_transform.translation == snapped_cursor_position {
            return;
        }

        let would_move_over_invalid_position = straddles_bag_or_overlaps_pieces(
            &rapier_context,
            Transform::from_translation(snapped_cursor_position).with_rotation(
                original
                    .map(|o| o.rotation)
                    .unwrap_or(global_transform.rotation),
            ),
            collider,
            piece,
        );

        if would_move_over_invalid_position {
            return;
        }

        if let Some(original) = original {
            piece_transform.rotation = original.rotation;
            commands
                .entity(piece)
                .remove_bundle::<UndoableAnimationBundle<Transform>>();
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
                Some(&|entity| entity != self_id),
            )
        })
        .is_some()
}

fn piece_is_floating(
    rapier_context: &Res<RapierContext>,
    transform: Transform,
    collider: &Collider,
    self_id: Entity,
) -> bool {
    // Check that the piece isn't floating by seeing if moving it down one unit
    // intersects with another piece.
    rapier_context
        .intersection_with_shape(
            transform.translation - const_vec3!([0., 0.5, 0.]),
            transform.rotation,
            collider,
            FLOATING_PIECE_COLLIDER_GROUP.into(),
            Some(&|entity| entity != self_id),
        )
        .is_none()
}
