use bevy::{prelude::*, window::WindowResized};
use bevy_prototype_lyon::draw::DrawMode;
use smallvec::SmallVec;

use crate::{
    animations,
    animations::{GameSpeed, Target},
    conveyor_belt::{
        consts::{HEIGHT, LENGTH, MAX_NUM_PIECES, PIECE_WIDTH},
        spawn::{ConveyorBeltInstance, ConveyorBeltMarker},
        ConveyorBelt, ConveyorBeltOptions,
    },
    levels::LevelStarted,
    nominos::{
        AttemptedPlacement, NominoMarker, NominoSpawner, PiecePlaced, PieceSystems, Selectable,
        Selected,
    },
    robot,
    robot::{RobotMarker, RobotTiming},
    window_management::{DipsWindow, WindowSystems},
};

const SELECTABLE_SEPARATION: f32 = 2.;
const NON_SELECTABLE_LIGHTNESS: f32 = 0.38;

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
        app.add_system(reposition_pieces_on_window_resize.after(WindowSystems));
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
    robot_timing: Query<&RobotTiming, With<RobotMarker>>,
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
                    let mut color = fill_mode.color.as_hsla();
                    if let Color::Hsla { lightness, .. } = &mut color {
                        *lightness = 0.5;
                    } else {
                        unreachable!()
                    }
                    fill_mode.color = color;
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

                let ttl = robot_timing
                    .get_single()
                    .map(|r| r.time_left())
                    .unwrap_or(robot::PLACEMENT_TTL);
                commands
                    .entity(spawned)
                    .insert_bundle(animations::piece_movement(from, target, ttl, &game_speed));
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
        let transform =
            Transform::from_translation(piece_position(&dips_window, &belt_options, position));
        let from = piece_positions.get(**attempted).unwrap();

        commands
            .entity(**attempted)
            .remove::<Selected>()
            .insert(animations::undo_selection(*from, transform, &game_speed));
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
    robot_timing: Query<&RobotTiming, With<RobotMarker>>,
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

        let ttl = robot_timing
            .get_single()
            .map(|r| r.time_left())
            .unwrap_or(robot::PLACEMENT_TTL);
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
                        ttl,
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
    robot_timing: Query<&RobotTiming, With<RobotMarker>>,
    mut piece_positions: Query<
        (&mut Transform, Option<&Target<Transform>>),
        (With<NominoMarker>, Without<Selected>),
    >,
) {
    if resized_events.iter().count() == 0 {
        return;
    }

    if let Ok(pieces) = belt_pieces.get_single() {
        for (index, piece) in pieces.iter().enumerate() {
            let position = piece_position(&dips_window, &belt_options, index);
            if let Ok((mut transform, target)) = piece_positions.get_mut(*piece) {
                if let Some(target) = target {
                    let diff = position - target.translation;
                    transform.translation += diff;

                    let ttl = robot_timing
                        .get_single()
                        .map(|r| r.time_left())
                        .unwrap_or(robot::PLACEMENT_TTL);
                    commands
                        .entity(*piece)
                        .insert_bundle(animations::piece_movement(
                            *transform,
                            transform.with_translation(position),
                            ttl,
                            &game_speed,
                        ));
                } else {
                    transform.translation = position;
                }
            }
        }
    }
}

fn piece_position(
    dips_window: &Res<DipsWindow>,
    belt_options: &Res<ConveyorBeltOptions>,
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

fn faded_piece_color(from: Color) -> Color {
    let mut color = from.as_hsla();
    if let Color::Hsla { lightness, .. } = &mut color {
        *lightness = NON_SELECTABLE_LIGHTNESS;
    } else {
        unreachable!()
    }
    color
}

fn maybe_spawn_piece(
    commands: &mut Commands,
    transform: Transform,
    position: usize,
    root: Entity,
    conveyor_belt: &mut dyn ConveyorBelt,
    belt_options: &Res<ConveyorBeltOptions>,
) -> Option<Entity> {
    conveyor_belt.next().map(|piece| {
        let color = if position < belt_options.num_pieces_selectable.into() {
            piece.color.render()
        } else {
            faded_piece_color(piece.color.render())
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
