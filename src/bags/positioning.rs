use bevy::{prelude::*, window::WindowResized};
use smallvec::SmallVec;

use crate::{
    bags::{
        bag_replacement::{BagPieces, BagSetupSystem},
        consts::BAG_SPACING,
        BagMarker, BagSize,
    },
    conveyor_belt,
    nominos::{NominoMarker, PiecePlaced},
    window_management::{DipsWindow, WindowSystems},
};

pub struct BagPositioningPlugin;

impl Plugin for BagPositioningPlugin {
    fn build(&self, app: &mut App) {
        // TODO add back after https://github.com/dimforge/bevy_rapier/issues/172
        // app.add_system_to_stage(
        //     CoreStage::PostUpdate,
        //     transfer_piece_ownership.after(TransformSystem::TransformPropagate),
        // );
        app.add_system(center_bags.after(WindowSystems).after(BagSetupSystem));
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

pub fn compute_bag_coordinates(
    window: &DipsWindow,
    bag_sizes: impl IntoIterator<Item = BagSize> + Clone,
) -> SmallVec<[Vec3; 3]> {
    let mut space_needed = 0;
    let mut max_half_height = 0.;
    for size in bag_sizes.clone() {
        space_needed += size.width() + BAG_SPACING;
        if size.half_height() > max_half_height {
            max_half_height = size.half_height();
        }
    }
    space_needed -= BAG_SPACING;

    let mut starting_position = BagCoord((window.width - space_needed as f32) / 2.).snap_to_grid();
    debug_assert!(starting_position >= 0. && starting_position <= window.width);

    let base_y = BagCoord((window.height - conveyor_belt::HEIGHT) / 2.).snap_to_grid();

    let mut bags = SmallVec::new();
    for bag in bag_sizes {
        starting_position += bag.half_width();
        bags.push(Vec3::new(
            starting_position,
            base_y - (max_half_height - bag.half_height()),
            0.,
        ));
        starting_position += bag.half_width() + BAG_SPACING as f32;
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
    mut bags: Query<
        (&mut Transform, &BagPieces, &BagSize),
        (With<BagMarker>, Without<NominoMarker>),
    >,
) {
    if resized_events.iter().count() == 0 {
        return;
    }
    if bags.is_empty() {
        return;
    }

    let bag_positions = compute_bag_coordinates(
        &dips_window,
        bags.iter().map(|bag| *bag.2).collect::<SmallVec<[_; 3]>>(),
    );

    for (index, (mut bag_position, bag_pieces, _)) in bags.iter_mut().enumerate() {
        let old_position = bag_position.translation;
        bag_position.translation = bag_positions[index];
        let diff = bag_position.translation - old_position;

        for piece in &**bag_pieces {
            piece_positions.get_mut(*piece).unwrap().translation += diff;
        }
    }
}
