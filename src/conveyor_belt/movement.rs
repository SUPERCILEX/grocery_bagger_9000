use bevy::{prelude::*, window::WindowResized};
use bevy_prototype_lyon::{draw::DrawMode, entity::Path};
use bevy_tweening::{AnimationSystem, Animator};
use smallvec::SmallVec;

use crate::{
    animations,
    animations::{GameSpeed, Target},
    conveyor_belt::{
        consts::{
            HEIGHT, LENGTH, MAX_NUM_PIECES, NON_SELECTABLE_LIGHTNESS, PIECE_WIDTH,
            SELECTABLE_SEPARATION,
        },
        positioning::compute_selectable_background,
        spawn::{
            nonselectable_background_path, selectable_background_path,
            BeltNonselectableBackgroundMarker, BeltSelectableBackgroundMarker,
            ConveyorBeltBackgroundSpawner, ConveyorBeltHackMarker, ConveyorBeltInstance,
            ConveyorBeltMarker,
        },
        ConveyorBelt, ConveyorBeltOptions,
    },
    gb9000::GroceryBagger9000,
    levels::LevelStarted,
    nominos::{
        AttemptedPlacement, NominoMarker, NominoSpawner, PiecePlaced, PieceSystems, Selectable,
        Selected, DEG_90, DEG_MIRRORED,
    },
    window_management::{DipsWindow, WindowSystems},
};

pub struct ConveyorBeltMovementPlugin;

impl Plugin for ConveyorBeltMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BeltEmptyEvent>();

        app.add_system_to_stage(CoreStage::PostUpdate, init_pieces);
        app.add_system(replace_pieces.after(WindowSystems).after(PieceSystems));
        app.add_system(belt_empty_check.after(replace_pieces));
        app.add_system(
            check_for_piece_selection_undos
                .after(WindowSystems)
                .after(PieceSystems)
                .after(replace_pieces),
        );
        app.add_system(move_pieces.after(WindowSystems).after(replace_pieces));
        app.add_system(
            reposition_pieces_on_window_resize
                .after(WindowSystems)
                .after(AnimationSystem::AnimationUpdate),
        );
        app.add_system(update_background_on_num_selectable_pieces_changed);
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            update_piece_selectability_on_num_selectable_pieces_changed,
        );
    }
}

pub struct BeltEmptyEvent;

#[derive(Default, Component, Deref, DerefMut)]
pub struct BeltPieceIds(SmallVec<[Entity; MAX_NUM_PIECES]>);

fn init_pieces(
    mut commands: Commands,
    mut level_loaded: EventReader<LevelStarted>,
    mut conveyor_belt: Query<
        (Entity, &mut ConveyorBeltInstance, &mut BeltPieceIds),
        With<ConveyorBeltMarker>,
    >,
    conveyor_belt_hack: Query<Entity, With<ConveyorBeltHackMarker>>,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    belt_options: Res<ConveyorBeltOptions>,
) {
    if level_loaded.iter().count() == 0 {
        return;
    }

    let (id, mut conveyor_belt, mut belt_pieces) = conveyor_belt.single_mut();
    for i in 0..MAX_NUM_PIECES {
        let start = Transform::from_xyz(
            dips_window.width + PIECE_WIDTH,
            dips_window.height - HEIGHT + PIECE_WIDTH,
            0.,
        );
        let spawned = maybe_spawn_piece(
            &mut commands,
            start,
            i,
            id,
            &mut ***conveyor_belt,
            &belt_options,
        );

        if let Some(spawned) = spawned {
            belt_pieces.push(spawned);
            commands.entity(spawned).insert(animations::piece_loaded(
                i,
                start,
                Transform::from_translation(piece_position(&dips_window, &belt_options, i)),
                &game_speed,
            ));
        } else {
            break;
        }
    }

    commands
        .entity(conveyor_belt_hack.single())
        .with_children(|parent| parent.spawn_belt_background(belt_options.num_pieces_selectable));
}

fn replace_pieces(
    mut commands: Commands,
    mut conveyor_belt: Query<
        (Entity, &mut ConveyorBeltInstance, &mut BeltPieceIds),
        With<ConveyorBeltMarker>,
    >,
    mut colors: Query<&mut DrawMode, (With<NominoMarker>, Without<Selectable>)>,
    mut placed_pieces: EventReader<PiecePlaced>,
    belt_options: Res<ConveyorBeltOptions>,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
) {
    for PiecePlaced {
        piece: piece_id, ..
    } in placed_pieces.iter()
    {
        let (id, mut conveyor_belt, mut belt_pieces) = conveyor_belt.single_mut();

        let placed_position = belt_pieces.iter().position(|id| *id == *piece_id);
        if let Some(placed_position) = placed_position {
            belt_pieces.remove(placed_position);
            if belt_options.num_pieces_selectable > 1 &&
            let Some(id) = belt_pieces.get(belt_options.num_pieces_selectable as usize - 1)
            {
                commands.entity(*id).insert(Selectable);

                let mut draw_mode = colors.get_mut(*id).unwrap();
                if let DrawMode::Outlined {
                    ref mut fill_mode, ..
                } = *draw_mode
                {
                    fill_mode.color = fill_mode.color.with_lightness(0.5);
                }
            }

            let position = MAX_NUM_PIECES - 1;
            let target =
                Transform::from_translation(piece_position(&dips_window, &belt_options, position));
            let from = {
                let mut from = target;
                from.translation.x = dips_window.width + PIECE_WIDTH;
                from
            };

            let spawned = maybe_spawn_piece(
                &mut commands,
                from,
                position,
                id,
                &mut ***conveyor_belt,
                &belt_options,
            );
            if let Some(spawned) = spawned {
                belt_pieces.push(spawned);

                commands
                    .entity(spawned)
                    .insert_bundle(animations::piece_movement(from, target, &game_speed));
            }
        }
    }
}

fn belt_empty_check(
    belt_pieces: Query<(&BeltPieceIds, ChangeTrackers<BeltPieceIds>), With<ConveyorBeltMarker>>,
    mut belt_empty: EventWriter<BeltEmptyEvent>,
) {
    if let Ok((belt_pieces, belt_changes)) = belt_pieces.get_single() {
        if !belt_changes.is_changed() {
            return;
        }

        if belt_pieces.is_empty() {
            belt_empty.send(BeltEmptyEvent);
        }
    }
}

fn check_for_piece_selection_undos(
    mut commands: Commands,
    mut attempted_placement_events: EventReader<AttemptedPlacement>,
    belt_pieces: Query<&BeltPieceIds, With<ConveyorBeltMarker>>,
    piece_positions: Query<&Transform, (With<NominoMarker>, With<Selected>)>,
    gb9000: Res<GroceryBagger9000>,
    belt_options: Res<ConveyorBeltOptions>,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
) {
    for attempted in attempted_placement_events.iter() {
        let position = belt_pieces
            .single()
            .iter()
            .position(|p| *p == **attempted)
            .unwrap();
        let from = piece_positions.get(**attempted).unwrap();
        let mut transform =
            Transform::from_translation(piece_position(&dips_window, &belt_options, position))
                .with_rotation(from.rotation);

        let mut unmirrored_rotation = transform.rotation;
        if transform.rotation.x.abs() > 1e-5 || transform.rotation.y.abs() > 1e-5 {
            unmirrored_rotation *= *DEG_MIRRORED;
        }
        if gb9000.current_level != 0
            && unmirrored_rotation.z.abs() > 1e-5
            && unmirrored_rotation.w.abs() > 1e-5
        {
            transform.rotation *= *DEG_90;
        }

        commands
            .entity(**attempted)
            .remove::<Selected>()
            .insert(animations::undo_selection(*from, transform, &game_speed));
    }
}

fn update_background_on_num_selectable_pieces_changed(
    belt_options: Res<ConveyorBeltOptions>,
    mut selectable: Query<
        &mut Path,
        (
            With<BeltSelectableBackgroundMarker>,
            Without<BeltNonselectableBackgroundMarker>,
        ),
    >,
    mut nonselectable: Query<
        (&mut Path, &mut Transform),
        (
            With<BeltNonselectableBackgroundMarker>,
            Without<BeltSelectableBackgroundMarker>,
        ),
    >,
) {
    if !belt_options.is_changed() {
        return;
    }

    let num_pieces_selectable = belt_options.num_pieces_selectable;
    if let Ok(mut path) = selectable.get_single_mut() {
        *path = selectable_background_path(num_pieces_selectable);
    }
    if let Ok((mut path, mut transform)) = nonselectable.get_single_mut() {
        *path = nonselectable_background_path(num_pieces_selectable);
        *transform = compute_selectable_background(num_pieces_selectable);
    }
}

fn update_piece_selectability_on_num_selectable_pieces_changed(
    mut commands: Commands,
    belt_options: Res<ConveyorBeltOptions>,
    conveyor_belt: Query<&BeltPieceIds, With<ConveyorBeltMarker>>,
    mut colors: Query<&mut DrawMode, With<NominoMarker>>,
) {
    if !belt_options.is_changed() {
        return;
    }

    let num_pieces_selectable = belt_options.num_pieces_selectable as usize;
    if let Ok(belt_pieces) = conveyor_belt.get_single() {
        for (index, piece) in belt_pieces.iter().enumerate() {
            let mut draw_mode = colors.get_mut(*piece).unwrap();
            if let DrawMode::Outlined {
                ref mut fill_mode, ..
            } = *draw_mode
            {
                if index < num_pieces_selectable {
                    fill_mode.color = fill_mode.color.with_lightness(0.5);
                    commands.entity(*piece).insert(Selectable);
                } else {
                    fill_mode.color = fill_mode.color.with_lightness(NON_SELECTABLE_LIGHTNESS);
                    commands.entity(*piece).remove::<Selectable>();
                };
            }
        }
    }
}

#[derive(Default, Eq, PartialEq)]
enum PieceMovementFsm {
    #[default]
    Ready,
    Loaded,
}

fn move_pieces(
    mut commands: Commands,
    belt_options: Res<ConveyorBeltOptions>,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    belt_pieces: Query<(&BeltPieceIds, ChangeTrackers<BeltPieceIds>), With<ConveyorBeltMarker>>,
    mut fsm: Local<PieceMovementFsm>,
    mut level_loaded: EventReader<LevelStarted>,
    positions: Query<&Transform, (With<NominoMarker>, Without<Selected>)>,
) {
    if level_loaded.iter().count() > 0 {
        *fsm = PieceMovementFsm::Ready;
    }

    if let Ok((belt_pieces, belt_changes)) = belt_pieces.get_single() {
        if !belt_changes.is_changed() {
            return;
        }
        if *fsm == PieceMovementFsm::Ready {
            *fsm = PieceMovementFsm::Loaded;
            return;
        }

        for (index, piece) in belt_pieces.iter().enumerate() {
            if let Ok(position) = positions.get(*piece) {
                commands
                    .entity(*piece)
                    .insert_bundle(animations::piece_movement(
                        *position,
                        Transform::from_translation(piece_position(
                            &dips_window,
                            &belt_options,
                            index,
                        )),
                        &game_speed,
                    ));
            }
        }
    }
}

fn reposition_pieces_on_window_resize(
    mut commands: Commands,
    mut resized_events: EventReader<WindowResized>,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
    belt_options: Res<ConveyorBeltOptions>,
    belt_pieces: Query<&BeltPieceIds, With<ConveyorBeltMarker>>,
    mut piece_positions: Query<
        (
            &mut Transform,
            Option<&Animator<Transform>>,
            Option<&Target<Transform>>,
        ),
        (With<NominoMarker>, Without<Selected>),
    >,
) {
    if resized_events.iter().count() == 0 {
        return;
    }

    if let Ok(pieces) = belt_pieces.get_single() {
        for (index, piece) in pieces.iter().enumerate() {
            let position = piece_position(&dips_window, &belt_options, index);
            if let Ok((mut transform, animator, target)) = piece_positions.get_mut(*piece) {
                if let Some(target) = target {
                    let diff = position - target.translation;
                    transform.translation += diff;

                    commands
                        .entity(*piece)
                        .insert_bundle(animations::piece_movement(
                            *transform,
                            transform.with_translation(position),
                            &game_speed,
                        ));
                } else if let Some(old_animator) = animator {
                    if old_animator.progress() > 1. - 1e-5 {
                        transform.translation = position;
                    } else {
                        let start = Transform::from_xyz(
                            dips_window.width + PIECE_WIDTH,
                            dips_window.height - HEIGHT + PIECE_WIDTH,
                            0.,
                        );
                        let mut animator = animations::piece_loaded(
                            index,
                            start,
                            Transform::from_translation(position),
                            &game_speed,
                        );
                        animator.set_progress(old_animator.progress());

                        commands.entity(*piece).insert(animator);
                    }
                } else {
                    transform.translation = position;
                }
            }
        }
    }
}

fn piece_position(
    dips_window: &DipsWindow,
    belt_options: &ConveyorBeltOptions,
    index: usize,
) -> Vec3 {
    let selectable_spacing = if index < belt_options.num_pieces_selectable.into() {
        SELECTABLE_SEPARATION
    } else {
        0.
    };

    let base = Vec2::new(dips_window.width - LENGTH, dips_window.height - HEIGHT);
    let offset = Vec2::new(index as f32 * PIECE_WIDTH - selectable_spacing, PIECE_WIDTH);
    (base + offset).round().extend(0.01)
}

fn maybe_spawn_piece(
    commands: &mut Commands,
    transform: Transform,
    position: usize,
    root: Entity,
    conveyor_belt: &mut dyn ConveyorBelt,
    belt_options: &ConveyorBeltOptions,
) -> Option<Entity> {
    conveyor_belt.next().map(|piece| {
        let color = if position < belt_options.num_pieces_selectable.into() {
            piece.color.render()
        } else {
            piece
                .color
                .render()
                .with_lightness(NON_SELECTABLE_LIGHTNESS)
        };

        commands
            .entity(root)
            .with_children(|parent| {
                let mut commands = parent.spawn_nomino(
                    transform.with_rotation(piece.rotation),
                    piece.nomino,
                    piece.color,
                    color,
                );
                if position < belt_options.num_pieces_selectable.into() {
                    commands.insert(Selectable);
                }
                commands.id()
            })
            .out
    })
}

trait ColorUtils {
    fn with_lightness(&self, value: f32) -> Color;
}

impl ColorUtils for Color {
    fn with_lightness(&self, value: f32) -> Color {
        let mut color = self.as_hsla();
        if let Color::Hsla { lightness, .. } = &mut color {
            *lightness = value;
        } else {
            unreachable!()
        }
        color
    }
}
