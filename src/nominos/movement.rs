use bevy::{math::const_vec3, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_tweening::AnimationSystem;
use smallvec::SmallVec;

use crate::{
    animations,
    animations::{GameSpeed, Original, UndoableAnimationBundle},
    bags::{BAG_BOUNDARY_COLLIDER_GROUP, BAG_COLLIDER_GROUP, BAG_FLOOR_COLLIDER_GROUP},
    levels::LevelMarker,
    nominos::*,
    window_management::{DipsWindow, MainCamera, WindowSystems},
    window_utils::compute_cursor_position,
};

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PiecePickedUp>();
        app.add_event::<AttemptedPlacement>();
        app.add_event::<PiecePlaced>();

        app.add_system(piece_selection_handler.label(PieceSystems));
        app.add_system(
            piece_rotation_handler
                .label(PieceSystems)
                .after(AnimationSystem::AnimationUpdate),
        );
        app.add_system(
            selected_piece_mover
                .label(PieceSystems)
                .after(WindowSystems)
                .before(piece_selection_handler)
                .after(AnimationSystem::AnimationUpdate),
        );
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct PieceSystems;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

#[derive(Deref)]
pub struct PiecePickedUp(Entity);

#[derive(Deref)]
pub struct AttemptedPlacement(Entity);

pub struct PiecePlaced {
    pub piece: Entity,
    pub bag: Entity,
}

const FLOATING_PIECE_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: BAG_FLOOR_COLLIDER_GROUP.memberships | NOMINO_COLLIDER_GROUP.memberships,
    filters: BAG_FLOOR_COLLIDER_GROUP.filters | NOMINO_COLLIDER_GROUP.filters,
};

fn piece_selection_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut picked_up_events: EventWriter<PiecePickedUp>,
    mut placed_events: EventWriter<PiecePlaced>,
    mut attempted_placement_events: EventWriter<AttemptedPlacement>,
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

            if let Some(bag) = intersects_with_bag {
                if !straddles_bag_or_overlaps_pieces(&rapier_context, *transform, collider, piece)
                    && !piece_is_floating(&rapier_context, *transform, collider, piece)
                {
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
            } else {
                attempted_placement_events.send(AttemptedPlacement(piece));
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
                #[cfg(not(feature = "debug"))]
                let selectable = selectables.contains(id);
                #[cfg(feature = "debug")]
                let selectable = debug_options.unrestricted_pieces || selectables.contains(id);

                if selectable {
                    let piece_positions = pieces_queries.p1();
                    let (piece_position, ..) = piece_positions.get(id).unwrap();

                    picked_up_events.send(PiecePickedUp(id));

                    commands
                        .entity(id)
                        .insert(LevelMarker)
                        .insert(Selected)
                        .insert(
                            piece_position.with_translation(
                                cursor_position.extend(piece_position.translation.z),
                            ),
                        )
                        .remove::<Parent>()
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
        if rotation.x.abs() > 1e-5 || rotation.y.abs() > 1e-5 {
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
    mut last_snapped_cursor_position: Local<Vec2>,
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

        let snapped_cursor_position = cursor_position.round();

        if *last_snapped_cursor_position == snapped_cursor_position {
            return;
        }
        *last_snapped_cursor_position = snapped_cursor_position;

        let mut nearby_positions = SmallVec::<[_; 9]>::new();
        for i in -1i8..=1 {
            for j in -1i8..=1 {
                nearby_positions.push(snapped_cursor_position + Vec2::new(f32::from(i), f32::from(j)));
            }
        }
        nearby_positions.sort_unstable_by(|a, b| {
            a.distance(cursor_position)
                .total_cmp(&b.distance(cursor_position))
        });

        let rotation = original.map_or(global_transform.rotation, |o| o.rotation);
        for position in nearby_positions {
            let snapped_cursor_position = position.extend(piece_transform.translation.z);
            let would_move_over_invalid_position = straddles_bag_or_overlaps_pieces(
                &rapier_context,
                Transform::from_translation(snapped_cursor_position).with_rotation(rotation),
                collider,
                piece,
            );
            if would_move_over_invalid_position {
                continue;
            }

            if let Some(original) = original {
                piece_transform.rotation = original.rotation;
                commands
                    .entity(piece)
                    .remove_bundle::<UndoableAnimationBundle<Transform>>();
            }
            piece_transform.translation = snapped_cursor_position;

            break;
        }
    }
}

fn straddles_bag_or_overlaps_pieces(
    rapier_context: &RapierContext,
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
    rapier_context: &RapierContext,
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
