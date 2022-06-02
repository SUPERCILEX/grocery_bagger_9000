use bevy::{prelude::*, window::WindowResized};
use smallvec::SmallVec;

use crate::{
    bags::{
        bag_replacement::{BagReplacementDetectionSystems, Exiting},
        consts::BAG_SPACING,
        spawn::BagContainerMarker,
        BagMarker, BagSize,
    },
    conveyor_belt,
    nominos::{NominoMarker, PiecePlaced, PieceSystems},
    window_management::{DipsWindow, WindowSystems},
};

pub struct BagPositioningPlugin;

impl Plugin for BagPositioningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(transfer_piece_ownership.after(PieceSystems));
        app.add_system(
            center_bags
                .after(WindowSystems)
                .after(BagReplacementDetectionSystems),
        );
    }
}

pub trait BagSnapper<T> {
    fn snap_to_grid(&self) -> T;
}

#[derive(Deref)]
pub struct BagCoord(pub f32);

impl BagSnapper<f32> for BagCoord {
    fn snap_to_grid(&self) -> f32 {
        if self.0 % 0.5 < 1e-5 {
            self.round() - 0.5
        } else {
            self.round() + 0.5
        }
    }
}

pub fn compute_container_coordinates(
    window: &DipsWindow,
    bag_sizes: impl IntoIterator<Item = BagSize>,
) -> Vec3 {
    let mut space_needed = 0;
    let mut max_half_height = 0.;
    for size in bag_sizes {
        space_needed += size.width() + BAG_SPACING;
        if size.half_height() > max_half_height {
            max_half_height = size.half_height();
        }
    }
    space_needed -= BAG_SPACING;

    let starting_position = BagCoord((window.width - f32::from(space_needed)) / 2.).snap_to_grid();
    let base_y = BagCoord((window.height - conveyor_belt::HEIGHT) / 2.).snap_to_grid();
    debug_assert!(starting_position >= 0. && starting_position <= window.width);
    debug_assert!(base_y >= 0. && base_y <= window.height);

    Vec3::new(starting_position, base_y, 0.)
}

fn transfer_piece_ownership(
    mut commands: Commands,
    mut piece_placements: EventReader<PiecePlaced>,
    bag_positions: Query<&GlobalTransform, With<BagMarker>>,
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

fn center_bags(
    mut resized_events: EventReader<WindowResized>,
    dips_window: Res<DipsWindow>,
    bags: Query<&BagSize, (With<BagMarker>, Without<Exiting>)>,
    mut container: Query<&mut Transform, (With<BagContainerMarker>, Without<NominoMarker>)>,
) {
    if resized_events.iter().count() == 0 {
        return;
    }
    if bags.is_empty() {
        return;
    }

    let base = compute_container_coordinates(
        &dips_window,
        bags.iter().copied().collect::<SmallVec<[_; 3]>>(),
    );

    container.single_mut().translation = base;
}
