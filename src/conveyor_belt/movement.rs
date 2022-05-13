use bevy::{prelude::*, transform::TransformSystem::TransformPropagate};
use bevy_prototype_lyon::draw::DrawMode;
use smallvec::SmallVec;

use crate::{
    conveyor_belt::{
        consts::{HEIGHT, LENGTH, MAX_NUM_PIECES, PIECE_WIDTH},
        spawn::{ConveyorBeltInstance, ConveyorBeltMarker},
        ConveyorBelt, ConveyorBeltOptions,
    },
    levels::{CurrentLevel, LevelLoaded},
    nominos::{NominoMarker, NominoSpawner, PiecePickedUp, Selectable},
    window_management::DipsWindow,
};

const SELECTABLE_SEPARATION: f32 = 2.;
const NON_SELECTABLE_LIGHTNESS: f32 = 0.38;

pub struct ConveyorBeltMovementPlugin;

impl Plugin for ConveyorBeltMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_pieces);
        app.add_system(replace_pieces.after(init_pieces));
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            move_pieces.before(TransformPropagate),
        );
    }
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct BeltPieceIds(SmallVec<[Entity; MAX_NUM_PIECES]>);

fn init_pieces(
    mut commands: Commands,
    mut level_loaded: EventReader<LevelLoaded>,
    mut conveyor_belt: Query<
        (&mut ConveyorBeltInstance, &mut BeltPieceIds),
        With<ConveyorBeltMarker>,
    >,
    dips_window: Res<DipsWindow>,
    belt_options: Res<ConveyorBeltOptions>,
) {
    if let Some(initialized_level) = level_loaded.iter().last() {
        let (mut conveyor_belt, mut belt_pieces) = conveyor_belt.single_mut();
        for i in 0..MAX_NUM_PIECES {
            let spawned = maybe_spawn_piece(
                &mut commands,
                i,
                **initialized_level,
                &mut ***conveyor_belt,
                &dips_window,
                &belt_options,
            );

            if let Some(spawned) = spawned {
                belt_pieces.push(spawned);
            } else {
                break;
            }
        }
    }
}

fn replace_pieces(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    mut conveyor_belt: Query<
        (&mut ConveyorBeltInstance, &mut BeltPieceIds),
        With<ConveyorBeltMarker>,
    >,
    mut colors: Query<&mut DrawMode, (With<NominoMarker>, Without<Selectable>)>,
    mut picked_up_pieces: EventReader<PiecePickedUp>,
    belt_options: Res<ConveyorBeltOptions>,
    dips_window: Res<DipsWindow>,
) {
    for piece_id in picked_up_pieces.iter() {
        let (mut conveyor_belt, mut belt_pieces) = conveyor_belt.single_mut();

        let picked_up_position = belt_pieces.iter().position(|id| *id == **piece_id);
        if let Some(picked_up_position) = picked_up_position {
            belt_pieces.remove(picked_up_position);
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

            let spawned = maybe_spawn_piece(
                &mut commands,
                MAX_NUM_PIECES - 1,
                current_level.root.unwrap(),
                &mut ***conveyor_belt,
                &dips_window,
                &belt_options,
            );
            if let Some(spawned) = spawned {
                belt_pieces.push(spawned);
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
    belt_options: Res<ConveyorBeltOptions>,
    dips_window: Res<DipsWindow>,
    belt_pieces: Query<(&BeltPieceIds, ChangeTrackers<BeltPieceIds>), With<ConveyorBeltMarker>>,
    mut fsm: Local<PieceMovementFsm>,
    mut level_loaded: EventReader<LevelLoaded>,
    mut positions: Query<&mut Transform, With<NominoMarker>>,
) {
    if level_loaded.iter().count() > 0 {
        *fsm = PieceMovementFsm::Ready;
    }

    let (belt_pieces, belt_changes) = belt_pieces.single();
    if !belt_changes.is_changed() {
        return;
    }
    if *fsm == PieceMovementFsm::Ready {
        *fsm = PieceMovementFsm::Loaded;
        return;
    }

    let base = Vec2::new(dips_window.width - LENGTH, dips_window.height - HEIGHT);
    for (index, piece) in belt_pieces.iter().enumerate() {
        let mut position = positions.get_mut(*piece).unwrap();
        position.translation = piece_position(&belt_options, index, base);
    }
}

fn piece_position(belt_options: &Res<ConveyorBeltOptions>, index: usize, base: Vec2) -> Vec3 {
    let selectable_spacing = if index < belt_options.num_pieces_selectable.into() {
        SELECTABLE_SEPARATION
    } else {
        0.
    };

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
    position: usize,
    root: Entity,
    conveyor_belt: &mut dyn ConveyorBelt,
    dips_window: &Res<DipsWindow>,
    belt_options: &Res<ConveyorBeltOptions>,
) -> Option<Entity> {
    conveyor_belt.next().map(|piece| {
        let color = if position < belt_options.num_pieces_selectable.into() {
            piece.color.render()
        } else {
            faded_piece_color(piece.color.render())
        };
        let base = Vec2::new(dips_window.width - LENGTH, dips_window.height - HEIGHT);

        commands
            .entity(root)
            .with_children(|parent| {
                let mut commands = parent.spawn_nomino(
                    Transform::from_translation(piece_position(belt_options, position, base))
                        .with_rotation(piece.rotation),
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
