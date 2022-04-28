use bevy::{prelude::*, render::camera::WindowOrigin, window::WindowMode};

const DEFAULT_WIDTH: f32 = 1200.;
const DEFAULT_HEIGHT: f32 = 675.;

const PIXELS_PER_UNIT: f32 = 30.;
const TARGET_WIDTH_UNITS: f32 = 48.;

pub struct WindowManager;

impl Plugin for WindowManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Grocery Bagger 9000".to_string(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            ..default()
        });
        app.insert_resource(ClearColor(Color::WHITE));
        app.add_startup_system(setup_cameras);
        app.add_system_to_stage(CoreStage::PreUpdate, monitor_scaling);
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
    projection_2d.single_mut().scale = if window_width >= TARGET_WIDTH_UNITS * PIXELS_PER_UNIT {
        1. / PIXELS_PER_UNIT
    } else {
        TARGET_WIDTH_UNITS / window_width
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
