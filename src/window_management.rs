use bevy::{prelude::*, render::camera::WindowOrigin, window::WindowMode};

use crate::window_utils::{PIXELS_PER_UNIT, TARGET_WIDTH_UNITS};

pub struct WindowManager;

impl Plugin for WindowManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Grocery Bagger 9000".to_string(),
            width: 1200.,
            height: 675.,
            ..default()
        });
        app.insert_resource(ClearColor(Color::WHITE));
        app.add_startup_system(setup_cameras);
        app.add_system(monitor_scaling);
        app.add_system(full_screen_toggle);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_web_resizer::Plugin);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_cameras(mut commands: Commands) {
    let mut camera_2d = OrthographicCameraBundle::new_2d();
    camera_2d.orthographic_projection.window_origin = WindowOrigin::BottomLeft;

    commands.spawn_bundle(camera_2d).insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
}

fn monitor_scaling(
    mut projection_2d: Query<&mut OrthographicProjection, With<MainCamera>>,
    windows: Res<Windows>,
) {
    if !windows.is_changed() {
        return;
    }

    let window_width = windows.get_primary().unwrap().width();
    let dips = window_width / PIXELS_PER_UNIT;
    projection_2d.single_mut().scale = if dips >= TARGET_WIDTH_UNITS {
        1. / PIXELS_PER_UNIT
    } else {
        1. / (window_width / TARGET_WIDTH_UNITS)
    };
}

fn full_screen_toggle(mut windows: ResMut<Windows>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::F) {
        let window = windows.get_primary_mut().unwrap();
        window.set_mode(if window.mode() == WindowMode::Windowed {
            WindowMode::BorderlessFullscreen
        } else {
            WindowMode::Windowed
        });
    }
}
