use bevy::{prelude::*, transform::TransformSystem::TransformPropagate};
use bevy_prototype_lyon::draw::DrawMode;

use crate::{
    conveyor_belt::{
        consts::{HEIGHT, LENGTH, MAX_NUM_PIECES, PIECE_WIDTH},
        ConveyorBeltInstance, ConveyorBeltOptions,
    },
    levels::{CurrentLevel, LevelLoaded, LevelUnloaded},
    nominos::{NominoMarker, NominoSpawner, PiecePickedUp, Selectable},
    window_management::DipsWindow,
};

const SELECTABLE_SEPARATION: f32 = 2.;
const NON_SELECTABLE_LIGHTNESS: f32 = 0.38;

pub struct ConveyorBeltMovementPlugin;

impl Plugin for ConveyorBeltMovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BeltPieceIds>();

        app.add_system_to_stage(CoreStage::PreUpdate, reset_conveyor_belt);
        app.add_system(init_pieces);
        app.add_system(replace_pieces.after(init_pieces));
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            move_pieces.before(TransformPropagate),
        );
        app.add_system_to_stage(CoreStage::PostUpdate, update_piece_selectability);
        app.add_system_to_stage(CoreStage::PostUpdate, fade_non_selectable_pieces);
    }
}

#[derive(Default, Deref, DerefMut)]
struct BeltPieceIds([Option<Entity>; MAX_NUM_PIECES]);

fn reset_conveyor_belt(
    current_level: Res<CurrentLevel>,
    mut level_unloaded: EventReader<LevelUnloaded>,
    mut conveyor_belt: ResMut<ConveyorBeltInstance>,
    mut belt_pieces: ResMut<BeltPieceIds>,
) {
    if level_unloaded.iter().count() > 0 && current_level.root.is_none() {
        **conveyor_belt = None;
        *belt_pieces = default();
    }
}

fn init_pieces(
    mut commands: Commands,
    mut level_initialized: EventReader<LevelLoaded>,
    mut conveyor_belt: ResMut<ConveyorBeltInstance>,
    mut belt_pieces: ResMut<BeltPieceIds>,
    belt_options: Res<ConveyorBeltOptions>,
    dips_window: Res<DipsWindow>,
) {
    // TODO these ANDs should be flipped, but CLion completely destroys the code if
    //  you do that and rustfmt is still too stupid to understand let chains.
    if let Some(conveyor_belt) = &mut **conveyor_belt &&
    let Some(initialized_level) = level_initialized.iter().last()
    {
        let base = Transform::from_xyz(dips_window.width - LENGTH, dips_window.height - HEIGHT, 0.);

        for piece_id in &mut **belt_pieces {
            if let Some(piece) = conveyor_belt.next() {
                let color = faded_piece_color(piece.color.render());

                commands
                    .entity(**initialized_level)
                    .with_children(|parent| {
                        let spawned = parent.spawn_nomino_with_color(
                            base,
                            piece.nomino,
                            piece.color,
                            color,
                            Transform::from_rotation(piece.rotation),
                        );

                        *piece_id = Some(spawned.id());
                    });
            } else {
                *piece_id = None;
            }
        }
    }
}

fn replace_pieces(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    mut conveyor_belt: ResMut<ConveyorBeltInstance>,
    mut belt_pieces: ResMut<BeltPieceIds>,
    mut picked_up_pieces: EventReader<PiecePickedUp>,
    belt_options: Res<ConveyorBeltOptions>,
    dips_window: Res<DipsWindow>,
) {
    for piece_id in picked_up_pieces.iter() {
        let picked_up_position = belt_pieces.iter().position(|id| id.contains(&**piece_id));
        if let Some(picked_up_position) = picked_up_position {
            for i in picked_up_position..MAX_NUM_PIECES - 1 {
                belt_pieces[i] = belt_pieces[i + 1];

                if i < belt_options.num_pieces_selectable.into() &&
                let Some(id) = belt_pieces[i]
                {
                    commands.entity(id).insert(Selectable);
                }
            }

            if let Some(conveyor_belt) = &mut **conveyor_belt &&
            let Some(piece) = conveyor_belt.next()
            {
                let color = faded_piece_color(piece.color.render());
                let base = Transform::from_xyz(
                    dips_window.width - LENGTH,
                    dips_window.height - HEIGHT,
                    0.,
                );

                commands
                    .entity(current_level.root.unwrap())
                    .with_children(|parent| {
                        let spawned = parent.spawn_nomino_with_color(
                            base,
                            piece.nomino,
                            piece.color,
                            color,
                            Transform::from_rotation(piece.rotation),
                        );

                        belt_pieces[MAX_NUM_PIECES - 1] = Some(spawned.id());
                    });
            } else {
                belt_pieces[MAX_NUM_PIECES - 1] = None;
            }
        }
    }
}

fn move_pieces(
    belt_pieces: Res<BeltPieceIds>,
    belt_options: Res<ConveyorBeltOptions>,
    mut positions: Query<&mut Transform, With<NominoMarker>>,
    dips_window: Res<DipsWindow>,
) {
    if !belt_pieces.is_changed() {
        return;
    }

    let base = Vec3::new(dips_window.width - LENGTH, dips_window.height - HEIGHT, 0.);
    for (index, piece) in belt_pieces.iter().enumerate() {
        if let Some(piece) = piece {
            let selectable_spacing = if index < belt_options.num_pieces_selectable.into() {
                SELECTABLE_SEPARATION
            } else {
                0.
            };

            let mut position = positions.get_mut(*piece).unwrap();
            position.translation = base
                + Vec3::new(
                    index as f32 * PIECE_WIDTH - selectable_spacing,
                    PIECE_WIDTH,
                    position.translation.z,
                );
        } else {
            break;
        }
    }
}

fn update_piece_selectability(
    mut commands: Commands,
    belt_pieces: Res<BeltPieceIds>,
    belt_options: Res<ConveyorBeltOptions>,
) {
    if !belt_options.is_changed() {
        return;
    }

    for (index, piece) in belt_pieces.iter().enumerate() {
        if let Some(piece) = piece {
            if index < belt_options.num_pieces_selectable.into() {
                commands.entity(*piece).insert(Selectable);
            } else {
                commands.entity(*piece).remove::<Selectable>();
            }
        }
    }
}

fn fade_non_selectable_pieces(
    belt_pieces: Res<BeltPieceIds>,
    belt_options: Res<ConveyorBeltOptions>,
    mut colors: Query<&mut DrawMode, With<NominoMarker>>,
) {
    if !belt_pieces.is_changed() {
        return;
    }

    let start = belt_options.num_pieces_selectable.into();

    for piece in &belt_pieces[..start] {
        if let Some(piece) = piece {
            let mut draw_mode = colors.get_mut(*piece).unwrap();
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
        } else {
            break;
        }
    }

    for piece in &belt_pieces[start..] {
        if let Some(piece) = piece {
            let mut draw_mode = colors.get_mut(*piece).unwrap();
            if let DrawMode::Outlined {
                ref mut fill_mode, ..
            } = *draw_mode
            {
                fill_mode.color = faded_piece_color(fill_mode.color);
            }
        } else {
            break;
        }
    }
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
