use bevy::{prelude::*, window::WindowResized};
use smallvec::SmallVec;

use crate::{
    bags::{
        consts::{BAG_SPACING, RADIUS},
        BagMarker,
    },
    conveyor_belt,
    nominos::PiecePlaced,
    window_management::DipsWindow,
};

pub struct BagPositioningPlugin;

impl Plugin for BagPositioningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(transfer_piece_ownership);
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

fn transfer_piece_ownership(
    mut commands: Commands,
    mut piece_placements: EventReader<PiecePlaced>,
    mut positions: Query<&mut Transform>,
    parents: Query<&Parent>,
) {
    for PiecePlaced { piece, bag } in piece_placements.iter() {
        if parents.get(*piece).map(|p| **p).contains(bag) {
            continue;
        }

        let bag_position = positions.get(*bag).unwrap().translation;
        positions.get_mut(*piece).unwrap().translation -= bag_position;

        commands.entity(*bag).add_child(*piece);
    }
}

fn center_bags(
    mut resized_events: EventReader<WindowResized>,
    dips_window: Res<DipsWindow>,
    mut bags: Query<&mut Transform, With<BagMarker>>,
) {
    if resized_events.iter().count() == 0 {
        return;
    }

    let bag_count = bags.iter().count();
    let bag_positions = compute_bag_coordinates(&dips_window, bag_count);
    for (index, mut bag_position) in bags.iter_mut().enumerate() {
        bag_position.translation = bag_positions[index];
    }
}
