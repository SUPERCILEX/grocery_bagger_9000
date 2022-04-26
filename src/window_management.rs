use bevy::{prelude::*, render::camera::WindowOrigin, window::WindowMode};

pub struct WindowManager;

const WIDTH_UNITS: u32 = 48;

impl Plugin for WindowManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Grocery Bagger 9000".to_string(),
            width: 1000.,
            height: 765.25,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_startup_system(setup_cameras)
        .add_system(monitor_scaling)
        .add_system(full_screen_toggle);
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
    projection_2d.single_mut().scale = 1. / (window_width / (WIDTH_UNITS as f32));
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
