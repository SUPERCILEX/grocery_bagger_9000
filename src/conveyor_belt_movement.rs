use bevy::prelude::*;

use crate::{
    conveyor_belt,
    conveyor_belt::ConveyorBeltInstance,
    levels::{CurrentLevel, LevelLoaded, LevelUnloaded},
    markers::Selectable,
    nominos::NominoSpawner,
    piece_movement::PiecePickedUp,
    window_management::DipsWindow,
};

pub struct ConveyorBeltMovementPlugin;

impl Plugin for ConveyorBeltMovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConveyorBeltInstance>();
        app.init_resource::<BeltPieceIds>();

        app.add_system_to_stage(CoreStage::PreUpdate, reset_conveyor_belt);
        app.add_system(init_pieces);
        app.add_system(replace_pieces.after(init_pieces));
        app.add_system_to_stage(CoreStage::PostUpdate, move_pieces);
    }
}

#[derive(Default, Deref, DerefMut)]
struct BeltPieceIds([Option<Entity>; conveyor_belt::MAX_NUM_PIECES]);

fn reset_conveyor_belt(
    current_level: Res<CurrentLevel>,
    mut level_unloaded: EventReader<LevelUnloaded>,
    mut conveyor_belt: ResMut<ConveyorBeltInstance>,
) {
    if level_unloaded.iter().count() > 0 && current_level.root.is_none() {
        **conveyor_belt = None;
    }
}

fn init_pieces(
    mut commands: Commands,
    mut level_initialized: EventReader<LevelLoaded>,
    mut conveyor_belt: ResMut<ConveyorBeltInstance>,
    mut belt_pieces: ResMut<BeltPieceIds>,
    dips_window: Res<DipsWindow>,
) {
    // TODO these ANDs should be flipped, but CLion completely destroys the code if you do that
    //  and rustfmt is still too stupid to understand let chains.
    if let Some(conveyor_belt) = &mut **conveyor_belt &&
    let Some(initialized_level) = level_initialized.iter().last()
    {
        let base = Transform::from_xyz(
            dips_window.width - conveyor_belt::LENGTH,
            dips_window.height - conveyor_belt::HEIGHT,
            0.,
        );

        for (index, piece_id) in (&mut **belt_pieces).iter_mut().enumerate() {
            if let Some(piece) = conveyor_belt.next() {
                commands
                    .entity(**initialized_level)
                    .with_children(|parent| {
                        let mut spawned = parent.spawn_nomino(
                            base,
                            piece.nomino,
                            piece.color,
                            Transform::from_xyz(
                                index as f32 * conveyor_belt::PIECE_WIDTH,
                                conveyor_belt::PIECE_WIDTH,
                                0.,
                            )
                                .with_rotation(piece.rotation),
                        );
                        if index < 3 {
                            spawned.insert(Selectable);
                        }

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
    dips_window: Res<DipsWindow>,
) {
    for piece_id in picked_up_pieces.iter() {
        let picked_up_position = belt_pieces.iter().position(|id| id.contains(&**piece_id));
        if let Some(picked_up_position) = picked_up_position {
            for i in picked_up_position..conveyor_belt::MAX_NUM_PIECES - 1 {
                belt_pieces[i] = belt_pieces[i + 1];

                if i < 3 &&
                let Some(id) = belt_pieces[i]
                {
                    commands.entity(id).insert(Selectable);
                }
            }

            if let Some(conveyor_belt) = &mut **conveyor_belt &&
            let Some(piece) = conveyor_belt.next()
            {
                let base = Transform::from_xyz(
                    dips_window.width - conveyor_belt::LENGTH,
                    dips_window.height - conveyor_belt::HEIGHT,
                    0.,
                );

                commands
                    .entity(current_level.root.unwrap())
                    .with_children(|parent| {
                        let spawned = parent.spawn_nomino(
                            base,
                            piece.nomino,
                            piece.color,
                            Transform::from_xyz(
                                (conveyor_belt::MAX_NUM_PIECES - 1) as f32
                                    * conveyor_belt::PIECE_WIDTH,
                                conveyor_belt::PIECE_WIDTH,
                                0.,
                            )
                                .with_rotation(piece.rotation),
                        );

                        belt_pieces[conveyor_belt::MAX_NUM_PIECES - 1] = Some(spawned.id());
                    });
            } else {
                belt_pieces[conveyor_belt::MAX_NUM_PIECES - 1] = None;
            }
        }
    }
}

fn move_pieces(
    belt_pieces: Res<BeltPieceIds>,
    mut positions: Query<&mut Transform>,
    dips_window: Res<DipsWindow>,
) {
    if !belt_pieces.is_changed() {
        return;
    }

    let base = Vec3::new(
        dips_window.width - conveyor_belt::LENGTH,
        dips_window.height - conveyor_belt::HEIGHT,
        0.,
    );
    for (index, piece) in belt_pieces.iter().enumerate() {
        if let Some(piece) = piece {
            let mut position = positions.get_mut(*piece).unwrap();
            position.translation = base
                + Vec3::new(
                    index as f32 * conveyor_belt::PIECE_WIDTH,
                    conveyor_belt::PIECE_WIDTH,
                    0.,
                );
        } else {
            break;
        }
    }
}