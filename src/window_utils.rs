use bevy::{prelude::*, render::camera::RenderTarget};

use crate::window_management::MainCamera;

pub const PIXELS_PER_UNIT: f32 = 25.;
pub const TARGET_WIDTH_UNITS: f32 = 48.;

pub fn compute_cursor_position(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera.single();
    let window = get_main_window(&windows, camera);

    window
        .cursor_position()
        .map(|position| window_to_world_coords(position, window, camera, camera_transform))
}

pub fn window_to_world_coords(
    coords: Vec2,
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (coords / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    world_pos.truncate()
}

fn get_main_window<'a>(windows: &'a Res<Windows>, camera: &Camera) -> &'a Window {
    if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    }
}
