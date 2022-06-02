use bevy::{math::const_vec3, prelude::*};
use bevy_prototype_lyon::{draw::DrawMode, entity::Path};
use bevy_tweening::Animator;
use smallvec::SmallVec;

use crate::{
    animations,
    animations::GameSpeed,
    colors::NominoColor,
    conveyor_belt::{
        consts::{
            LENGTH, MAX_NUM_PIECES, NON_SELECTABLE_LIGHTNESS, PIECE_WIDTH, SELECTABLE_SEPARATION,
        },
        positioning::compute_selectable_background,
        spawn::{
            nonselectable_background_path, selectable_background_path,
            BeltNonselectableBackgroundMarker, BeltSelectableBackgroundMarker,
            ConveyorBeltBackgroundSpawner, ConveyorBeltInstance, ConveyorBeltMarker,
        },
        ConveyorBelt, ConveyorBeltOptions,
    },
    gb9000::GroceryBagger9000,
    levels::{LevelStarted, LevelTransitionSystems},
    nominos::{
        NominoMarker, NominoSpawner, OutOfBagPlacement, PiecePickedUp, PiecePlaced, PieceSystems,
        Selectable, Selected, DEG_90, DEG_MIRRORED,
    },
    robot::RobotTargetMarker,
    ui::MenuButtonClickedSystems,
    window_management::WindowSystems,
};

pub struct ConveyorBeltMovementPlugin;

impl Plugin for ConveyorBeltMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BeltEmptyEvent>();

        app.add_system_to_stage(CoreStage::PostUpdate, init_pieces);
        app.add_system(
            replace_pieces
                .label(BeltMovementSystems)
                .after(WindowSystems)
                .after(PieceSystems),
        );
        app.add_system(belt_empty_check.after(replace_pieces));
        app.add_system(
            check_for_piece_selection_undos
                .after(WindowSystems)
                .after(PieceSystems)
                .after(MenuButtonClickedSystems)
                .before(LevelTransitionSystems)
                .after(replace_pieces),
        );
        app.add_system(
            move_pieces
                .label(BeltMovementSystems)
                .after(LevelTransitionSystems)
                .after(WindowSystems)
                .after(replace_pieces),
        );
        app.add_system(
            update_robot_target_on_piece_selection
                .after(PieceSystems)
                .after(replace_pieces),
        );
        app.add_system(update_background_on_num_selectable_pieces_changed);
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            update_piece_selectability_on_num_selectable_pieces_changed,
        );
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct BeltMovementSystems;

pub struct BeltEmptyEvent;

#[derive(Default, Component, Deref, DerefMut)]
pub struct BeltPieceIds(SmallVec<[Entity; MAX_NUM_PIECES as usize]>);

fn init_pieces(
    mut commands: Commands,
    mut level_loaded: EventReader<LevelStarted>,
    mut conveyor_belt: Query<
        (Entity, &mut ConveyorBeltInstance, &mut BeltPieceIds),
        With<ConveyorBeltMarker>,
    >,
    game_speed: Res<GameSpeed>,
    belt_options: Res<ConveyorBeltOptions>,
) {
    if level_loaded.iter().count() == 0 {
        return;
    }

    let (id, mut conveyor_belt, mut belt_pieces) = conveyor_belt.single_mut();
    for i in 0..MAX_NUM_PIECES {
        let start =
            Transform::from_translation(const_vec3!([LENGTH + PIECE_WIDTH, PIECE_WIDTH, 0.]));
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
                Transform::from_translation(piece_position(&belt_options, i)),
                &game_speed,
            ));
        } else {
            break;
        }
    }

    commands
        .entity(id)
        .with_children(|parent| parent.spawn_belt_background(belt_options.num_pieces_selectable));
}

fn replace_pieces(
    mut commands: Commands,
    mut conveyor_belt: Query<
        (Entity, &mut ConveyorBeltInstance, &mut BeltPieceIds),
        With<ConveyorBeltMarker>,
    >,
    mut colors: Query<&mut DrawMode, (With<NominoMarker>, Without<Selectable>)>,
    nomino_colors: Query<&NominoColor, With<NominoMarker>>,
    selected_pieces: Query<(), With<Selected>>,
    mut placed_pieces: EventReader<PiecePlaced>,
    belt_options: Res<ConveyorBeltOptions>,
    game_speed: Res<GameSpeed>,
) {
    for PiecePlaced {
        piece: piece_id, ..
    } in placed_pieces.iter()
    {
        let (id, mut conveyor_belt, mut belt_pieces) = conveyor_belt.single_mut();

        let placed_position = belt_pieces.iter().position(|id| *id == *piece_id);
        let placed_position = if let Some(placed_position) = placed_position {
            placed_position
        } else {
            continue;
        };

        belt_pieces.remove(placed_position);
        commands
            .entity(*piece_id)
            .remove::<Selectable>()
            .remove::<RobotTargetMarker>();
        if let Some(id) = belt_pieces
            .iter()
            .find(|id| !selected_pieces.contains(**id))
        {
            commands.entity(*id).insert(RobotTargetMarker);
        }

        if belt_options.num_pieces_selectable > 1 &&
        let Some(id) = belt_pieces.get(belt_options.num_pieces_selectable as usize - 1)
        {
            commands.entity(*id).insert(Selectable);

            let mut draw_mode = colors.get_mut(*id).unwrap();
            if let DrawMode::Outlined {
                ref mut fill_mode, ..
            } = *draw_mode
            {
                fill_mode.color = nomino_colors.get(*id).unwrap().render();
            }
        }

        let position = MAX_NUM_PIECES - 1;
        let target = Transform::from_translation(piece_position(&belt_options, position));
        let from = {
            let mut from = target;
            from.translation.x = LENGTH + PIECE_WIDTH;
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
    mut attempted_placement_events: EventReader<OutOfBagPlacement>,
    belt: Query<(Entity, &GlobalTransform, &BeltPieceIds), With<ConveyorBeltMarker>>,
    piece_positions: Query<&GlobalTransform, (With<NominoMarker>, With<Selected>)>,
    old_robot_target: Query<Entity, With<RobotTargetMarker>>,
    gb9000: Res<GroceryBagger9000>,
    belt_options: Res<ConveyorBeltOptions>,
    game_speed: Res<GameSpeed>,
) {
    for attempted in attempted_placement_events.iter() {
        let (id, belt_position, belt_pieces) = belt.single();
        let position =
            u8::try_from(belt_pieces.iter().position(|p| *p == **attempted).unwrap()).unwrap();
        let from = piece_positions.get(**attempted).unwrap();
        let mut transform = Transform::from_translation(piece_position(&belt_options, position))
            .with_rotation(from.rotation);

        let mut unmirrored_rotation = transform.rotation;
        if transform.rotation.x.abs() > 1e-5 || transform.rotation.y.abs() > 1e-5 {
            unmirrored_rotation *= *DEG_MIRRORED;
        }
        if gb9000.current_level != 1
            && unmirrored_rotation.z.abs() > 1e-5
            && unmirrored_rotation.w.abs() > 1e-5
        {
            transform.rotation *= *DEG_90;
        }

        commands.entity(id).add_child(**attempted);

        let from_translation = from.translation - belt_position.translation;
        let mut piece_commands = commands.entity(**attempted);
        piece_commands
            .remove::<Selected>()
            .insert_bundle(animations::undo_selection(
                from.with_translation(from_translation).into(),
                transform,
                &game_speed,
            ));
        if position == 0 {
            piece_commands.insert(RobotTargetMarker);
            if let Ok(id) = old_robot_target.get_single() {
                debug_assert_ne!(id, **attempted);
                commands.entity(id).remove::<RobotTargetMarker>();
            }
        }
    }
}

fn update_robot_target_on_piece_selection(
    mut commands: Commands,
    belt_pieces: Query<&BeltPieceIds, With<ConveyorBeltMarker>>,
    mut picked_up_pieces: EventReader<PiecePickedUp>,
) {
    for piece in picked_up_pieces.iter() {
        commands.entity(**piece).remove::<RobotTargetMarker>();
        if let Some(id) = belt_pieces.single().iter().find(|id| **id != **piece) {
            commands.entity(*id).insert(RobotTargetMarker);
        }
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
            Without<NominoMarker>,
            Without<Animator<Transform>>,
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
    nomino_colors: Query<&NominoColor, With<NominoMarker>>,
    mut colors: Query<&mut DrawMode, With<NominoMarker>>,
) {
    if !belt_options.is_changed() {
        return;
    }

    let num_pieces_selectable = belt_options.num_pieces_selectable as usize;
    let belt_pieces = if let Ok(belt_pieces) = conveyor_belt.get_single() {
        belt_pieces
    } else {
        return;
    };

    for (index, piece) in belt_pieces.iter().enumerate() {
        let mut draw_mode = colors.get_mut(*piece).unwrap();
        if let DrawMode::Outlined {
            ref mut fill_mode, ..
        } = *draw_mode
        {
            if index < num_pieces_selectable {
                fill_mode.color = nomino_colors.get(*piece).unwrap().render();
                commands.entity(*piece).insert(Selectable);
            } else {
                fill_mode.color = fill_mode.color.with_lightness(NON_SELECTABLE_LIGHTNESS);
                commands.entity(*piece).remove::<Selectable>();
            };
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
    game_speed: Res<GameSpeed>,
    belt_pieces: Query<(&BeltPieceIds, ChangeTrackers<BeltPieceIds>), With<ConveyorBeltMarker>>,
    mut fsm: Local<PieceMovementFsm>,
    mut level_loaded: EventReader<LevelStarted>,
    positions: Query<&Transform, (With<NominoMarker>, Without<Selected>)>,
) {
    if level_loaded.iter().count() > 0 {
        *fsm = PieceMovementFsm::Ready;
    }

    let (belt_pieces, belt_changes) = if let Ok(p) = belt_pieces.get_single() {
        p
    } else {
        return;
    };

    if !belt_changes.is_changed() {
        return;
    }
    if *fsm == PieceMovementFsm::Ready {
        *fsm = PieceMovementFsm::Loaded;
        return;
    }

    for (index, piece) in belt_pieces.iter().enumerate() {
        if let Ok(position) = positions.get(*piece) {
            let target = Transform::from_translation(piece_position(
                &belt_options,
                index.try_into().unwrap(),
            ));

            if !position.translation.abs_diff_eq(target.translation, 1e-3) {
                commands
                    .entity(*piece)
                    .insert_bundle(animations::piece_movement(*position, target, &game_speed));
            }
        }
    }
}

fn piece_position(belt_options: &ConveyorBeltOptions, index: u8) -> Vec3 {
    let selectable_spacing = if index < belt_options.num_pieces_selectable {
        SELECTABLE_SEPARATION
    } else {
        0.
    };

    Vec2::new(
        f32::from(index).mul_add(
            PIECE_WIDTH,
            2.5 * SELECTABLE_SEPARATION - selectable_spacing,
        ),
        PIECE_WIDTH,
    )
    .round()
    .extend(0.01)
}

fn maybe_spawn_piece(
    commands: &mut Commands,
    transform: Transform,
    position: u8,
    root: Entity,
    conveyor_belt: &mut dyn ConveyorBelt,
    belt_options: &ConveyorBeltOptions,
) -> Option<Entity> {
    conveyor_belt.next().map(|piece| {
        let color = if position < belt_options.num_pieces_selectable {
            piece.color.render()
        } else {
            piece
                .color
                .render()
                .with_lightness(NON_SELECTABLE_LIGHTNESS)
        };

        commands.entity(root).add_children(|parent| {
            let mut commands = parent.spawn_nomino(
                transform.with_rotation(piece.rotation),
                piece.nomino,
                piece.color,
                color,
            );

            if position < belt_options.num_pieces_selectable {
                commands.insert(Selectable);
            }
            if position == 0 {
                commands.insert(RobotTargetMarker);
            }

            commands.id()
        })
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
