use bevy::{prelude::*, window::WindowResized};

use crate::{
    conveyor_belt::{
        consts::{LENGTH, PIECE_WIDTH, SELECTABLE_SEPARATION},
        spawn::ConveyorBeltHackMarker,
        HEIGHT,
    },
    window_management::{DipsWindow, WindowSystems},
};

pub struct ConveyorBeltPositioningPlugin;

impl Plugin for ConveyorBeltPositioningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(reposition_background_on_window_resize.after(WindowSystems));
    }
}

pub fn compute_belt_position(dips_window: &DipsWindow) -> Transform {
    Transform::from_xyz(
        dips_window.width - (2f32.mul_add(SELECTABLE_SEPARATION, LENGTH + 0.5)),
        dips_window.height - HEIGHT,
        0.,
    )
}

pub fn compute_selectable_background(num_pieces_selectable: u8) -> Transform {
    Transform::from_xyz(
        f32::from(num_pieces_selectable).mul_add(
            PIECE_WIDTH,
            SELECTABLE_SEPARATION + SELECTABLE_SEPARATION / 2.,
        ),
        0.,
        0.,
    )
}

fn reposition_background_on_window_resize(
    mut resized_events: EventReader<WindowResized>,
    dips_window: Res<DipsWindow>,
    mut background: Query<&mut Transform, With<ConveyorBeltHackMarker>>,
) {
    if resized_events.iter().count() == 0 {
        return;
    }

    if let Ok(mut position) = background.get_single_mut() {
        *position = compute_belt_position(&dips_window);
    }
}
