use bevy::{
    prelude::*,
    render::camera::WindowOrigin,
    window::{WindowMode, WindowResized},
    winit::{UpdateMode, WinitSettings},
};

const DEFAULT_WIDTH: f32 = 1200.;
const DEFAULT_HEIGHT: f32 = 675.;

const PIXELS_PER_UNIT: f32 = 30.;
const TARGET_WIDTH_UNITS: f32 = 48.;
const TARGET_HEIGHT_UNITS: f32 = 16.;

pub struct WindowManager;

impl Plugin for WindowManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Grocery Bagger 9000".to_string(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            ..default()
        });
        app.insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            ..WinitSettings::desktop_app()
        });
        app.insert_resource(ClearColor(Color::WHITE));

        app.init_resource::<DipsWindow>();

        app.add_startup_system(setup);
        app.add_system_to_stage(CoreStage::PreUpdate, window_scaling);
        app.add_system(full_screen_toggle);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_web_resizer::Plugin);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Default)]
pub struct DipsWindow {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}

fn setup(mut commands: Commands, windows: Res<Windows>, mut dips_window: ResMut<DipsWindow>) {
    let mut camera_2d = OrthographicCameraBundle::new_2d();
    camera_2d.orthographic_projection.window_origin = WindowOrigin::BottomLeft;

    let primary_window = windows.get_primary().unwrap();
    scale_window(
        &mut camera_2d.orthographic_projection,
        &mut dips_window,
        primary_window.width(),
        primary_window.height(),
    );

    commands.spawn_bundle(camera_2d).insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
}

fn window_scaling(
    mut projection_2d: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut resized_events: EventReader<WindowResized>,
    mut dips_window: ResMut<DipsWindow>,
) {
    if let Some(primary_window) = resized_events.iter().filter(|w| w.id.is_primary()).last() {
        let mut proj = projection_2d.single_mut();
        scale_window(
            &mut proj,
            &mut dips_window,
            primary_window.width,
            primary_window.height,
        );
    }
}

fn scale_window(
    proj: &mut OrthographicProjection,
    dips_window: &mut DipsWindow,
    window_width: f32,
    window_height: f32,
) {
    let meets_width_requirement = window_width >= TARGET_WIDTH_UNITS * PIXELS_PER_UNIT;
    let meets_height_requirement = window_height >= TARGET_HEIGHT_UNITS * PIXELS_PER_UNIT;
    proj.scale = if meets_width_requirement && meets_height_requirement {
        1. / PIXELS_PER_UNIT
    } else {
        f32::max(
            TARGET_WIDTH_UNITS / window_width,
            TARGET_HEIGHT_UNITS / window_height,
        )
    };

    dips_window.width = window_width * proj.scale;
    dips_window.height = window_height * proj.scale;
    dips_window.scale = proj.scale;
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
