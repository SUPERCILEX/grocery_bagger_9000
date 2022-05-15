use bevy::{prelude::*, window::WindowResized};
use smallvec::SmallVec;

use crate::{
    bags::{
        bag_replacement::BagPieces,
        consts::{BAG_SPACING, RADIUS},
        BagMarker,
    },
    conveyor_belt,
    nominos::{NominoMarker, PiecePlaced},
    window_management::DipsWindow,
};

pub struct BagPositioningPlugin;

impl Plugin for BagPositioningPlugin {
    fn build(&self, app: &mut App) {
        // TODO add back after https://github.com/dimforge/bevy_rapier/issues/172
        // app.add_system_to_stage(
        //     CoreStage::PostUpdate,
        //     transfer_piece_ownership.after(TransformSystem::TransformPropagate),
        // );
        app.add_system(center_bags);
    }
}

pub trait BagSnapper<T> {
    fn snap_to_grid(&self) -> T;
}

#[derive(Deref)]
pub struct BagCoord(pub f32);

impl BagSnapper<f32> for BagCoord {
    fn snap_to_grid(&self) -> f32 {
        self.round() + 0.5
    }
}

pub fn compute_bag_coordinates(window: &DipsWindow, num_bags: usize) -> SmallVec<[Vec3; 3]> {
    debug_assert!(num_bags != 0);

    let space_needed = 2. * RADIUS * num_bags as f32 + (num_bags - 1) as f32 * BAG_SPACING;
    let starting_position = (window.width - space_needed) / 2. + RADIUS;
    debug_assert!(starting_position >= 0. && starting_position <= window.width);

    let mut bags = SmallVec::new();
    for bag in 0..num_bags {
        bags.push(Vec3::new(
            BagCoord(starting_position + (2. * RADIUS * bag as f32 + bag as f32 * BAG_SPACING))
                .snap_to_grid(),
            BagCoord((window.height - conveyor_belt::HEIGHT) / 2.).snap_to_grid(),
            0.,
        ))
    }
    bags
}

// TODO https://github.com/dimforge/bevy_rapier/issues/172
#[allow(dead_code)]
fn transfer_piece_ownership(
    mut commands: Commands,
    mut piece_placements: EventReader<PiecePlaced>,
    bag_positions: Query<&Transform, With<BagMarker>>,
    mut piece_positions: Query<
        (&GlobalTransform, &mut Transform),
        (With<NominoMarker>, Without<BagMarker>),
    >,
) {
    for PiecePlaced { piece, bag } in piece_placements.iter() {
        commands.entity(*bag).add_child(*piece);

        let bag_global = bag_positions.get(*bag).unwrap().translation;
        let (piece_global, mut piece_local) = piece_positions.get_mut(*piece).unwrap();
        piece_local.translation = piece_global.translation - bag_global;
    }
}

// TODO only move bags after https://github.com/dimforge/bevy_rapier/issues/172
fn center_bags(
    mut resized_events: EventReader<WindowResized>,
    dips_window: Res<DipsWindow>,
    mut piece_positions: Query<&mut Transform, (With<NominoMarker>, Without<BagMarker>)>,
    mut bags: Query<(&mut Transform, &BagPieces), (With<BagMarker>, Without<NominoMarker>)>,
) {
    if resized_events.iter().count() == 0 {
        return;
    }
    let bag_count = bags.iter().count();
    if bag_count == 0 {
        return;
    }

    let bag_positions = compute_bag_coordinates(&dips_window, bag_count);

    for (index, (mut bag_position, bag_pieces)) in bags.iter_mut().enumerate() {
        let old_position = bag_position.translation;
        bag_position.translation = bag_positions[index];
        let diff = bag_position.translation - old_position;

        for piece in &**bag_pieces {
            piece_positions.get_mut(*piece).unwrap().translation += diff;
        }
    }
}
