use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    animations,
    animations::{AnimationBundle, Original},
    bags::{BAG_BOUNDARY_COLLIDER_GROUP, BAG_COLLIDER_GROUP},
    levels::LevelUnloaded,
    nomino_consts::DEG_90,
    nominos::*,
    window_management::MainCamera,
    window_utils::compute_cursor_position,
};

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPiece>();
        app.add_event::<PiecePlaced>();
        app.add_event::<PiecePickedUp>();

        app.add_system_to_stage(CoreStage::PreUpdate, reset_selected_piece);
        app.add_system(piece_selection_handler);
        app.add_system(
            piece_rotation_handler.after(bevy_tweening::component_animator_system::<Transform>),
        );
        app.add_system(selected_piece_mover.before(piece_selection_handler));
    }
}

#[derive(Component)]
pub struct Selectable;

pub struct PiecePlaced {
    pub piece: Entity,
    pub bag: Entity,
}

#[derive(Deref)]
pub struct PiecePickedUp(Entity);

#[derive(Deref, DerefMut, Default)]
struct SelectedPiece(Option<Entity>);

fn reset_selected_piece(
    mut level_unloaded: EventReader<LevelUnloaded>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    if level_unloaded.iter().count() > 0 {
        **selected_piece = None;
    }
}

fn piece_selection_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut picked_up_events: EventWriter<PiecePickedUp>,
    mut placed_events: EventWriter<PiecePlaced>,
    selectables: Query<(), With<Selectable>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    rapier_context: Res<RapierContext>,
    mut selected_shape: Query<(&mut Transform, &Collider, Option<&Original<Transform>>)>,
    #[cfg(feature = "debug")] debug_options: Res<crate::debug::DebugOptions>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(piece) = &**selected_piece {
        let (mut transform, collider, original) = selected_shape.get_mut(*piece).unwrap();
        if let Some(original) = original {
            transform.rotation = original.rotation;
            commands
                .entity(*piece)
                .remove_bundle::<AnimationBundle<Transform>>();
        }

        #[cfg(feature = "debug")]
        if debug_options.unrestricted_pieces {
            *selected_piece = default();
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
            && !straddles_bag_or_overlaps_pieces(&rapier_context, *transform, collider, *piece)
            && !piece_is_floating(&rapier_context, *transform, collider, *piece) {
            commands.entity(*piece).remove::<Selectable>();

            placed_events.send(PiecePlaced {
                piece: *piece,
                bag,
            });
            *selected_piece = default();
        } else {
            commands.entity(*piece).insert_bundle(animations::error_shake(*transform));
        }

        return;
    }

    if let Some(cursor_position) = compute_cursor_position(windows, camera) {
        rapier_context.intersections_with_point(
            cursor_position.extend(0.),
            NOMINO_COLLIDER_GROUP.into(),
            Some(&|entity| {
                #[cfg(feature = "debug")]
                if debug_options.unrestricted_pieces {
                    return true;
                }

                selectables.contains(entity)
            }),
            |id| {
                *selected_piece = SelectedPiece(Some(id));
                picked_up_events.send(PiecePickedUp(id));
                false
            },
        );
    }
}

fn piece_rotation_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    selected_piece: Res<SelectedPiece>,
    mut pieces: Query<(&mut Transform, Option<&Original<Transform>>)>,
) {
    if !(mouse_button_input.just_pressed(MouseButton::Right)) {
        return;
    }

    if let Some(piece) = &**selected_piece {
        let (transform, original) = &mut pieces.get_mut(*piece).unwrap();
        if let Some(original) = original {
            transform.rotation = original.rotation;
            commands
                .entity(*piece)
                .remove_bundle::<AnimationBundle<Transform>>();
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
    selected_piece: Res<SelectedPiece>,
    mut pieces: Query<(&mut Transform, &Collider)>,
    rapier_context: Res<RapierContext>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Some(piece) = &**selected_piece &&
    let Some(cursor_position) = compute_cursor_position(windows, camera_query)
    {
        let (mut piece_transform, collider) = pieces.get_mut(*piece).unwrap();
        let snapped_cursor_position = cursor_position.round().extend(0.);

        let would_move_over_invalid_position = straddles_bag_or_overlaps_pieces(
            &rapier_context,
            Transform::from_translation(snapped_cursor_position).with_rotation(piece_transform.rotation),
            collider,
            *piece,
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
            transform.translation - Vec3::new(0., 0.5, 0.),
            transform.rotation,
            collider,
            CollisionGroups::new(0b101, 0b101).into(),
            Some(&|entity| entity != self_id),
        )
        .is_none()
}
