use bevy::{ecs::schedule::ShouldRun, prelude::*, window::WindowResized};
use bevy_tweening::Animator;

use crate::{
    conveyor_belt::{
        consts::{LENGTH, PIECE_WIDTH, SELECTABLE_SEPARATION},
        spawn::ConveyorBeltMarker,
        HEIGHT,
    },
    nominos::NominoMarker,
    window_management::{DipsWindow, WindowSystems},
};

pub struct ConveyorBeltPositioningPlugin;

impl Plugin for ConveyorBeltPositioningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            reposition_background_on_window_resize
                .with_run_criteria(run_if_window_resized)
                .after(WindowSystems),
        );
    }
}

pub fn compute_belt_position(dips_window: &DipsWindow) -> Transform {
    Transform::from_xyz(dips_window.width - LENGTH, dips_window.height - HEIGHT, 0.)
}

pub fn compute_selectable_background(num_pieces_selectable: u8) -> Transform {
    Transform::from_xyz(
        f32::from(num_pieces_selectable).mul_add(PIECE_WIDTH, 1.5 * SELECTABLE_SEPARATION),
        0.,
        0.,
    )
}

fn run_if_window_resized(mut resized_events: EventReader<WindowResized>) -> ShouldRun {
    if resized_events.iter().count() > 0 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn reposition_background_on_window_resize(
    dips_window: Res<DipsWindow>,
    mut background: Query<
        &mut Transform,
        (
            With<ConveyorBeltMarker>,
            Without<NominoMarker>,
            Without<Animator<Transform>>,
        ),
    >,
) {
    if let Ok(mut position) = background.get_single_mut() {
        *position = compute_belt_position(&dips_window);
    }
}
