use bevy::{
    prelude::*,
    render::camera::WindowOrigin,
    window::{WindowId, WindowMode, WindowResized},
};

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
        app.add_system_to_stage(CoreStage::PreUpdate, window_scaling);
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

fn window_scaling(
    mut projection_2d: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut resized_events: EventReader<WindowResized>,
) {
    if let Some(primary_window) = resized_events.iter().filter(|w| w.id.is_primary()).last() {
        projection_2d.single_mut().scale =
            if primary_window.width >= TARGET_WIDTH_UNITS * PIXELS_PER_UNIT {
                1. / PIXELS_PER_UNIT
            } else {
                TARGET_WIDTH_UNITS / primary_window.width
            };
    }
}

fn full_screen_toggle(mut windows: ResMut<Windows>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::F) {
        let window = windows.get_primary_mut().unwrap();
        window.set_mode(if window.mode() == WindowMode::Windowed {
            WindowMode::BorderlessFullscreen
        } else {
            WindowMode::Windowed
        });
    }
}
