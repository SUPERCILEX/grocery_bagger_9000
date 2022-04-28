#![feature(let_chains)]
#![feature(once_cell)]
#![allow(clippy::too_many_arguments)]

use bevy::{app::App, DefaultPlugins};
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::{gb9000::GroceryBagger9000Plugin, window_management::WindowManager};

mod bags;
mod conveyor_belt;
mod gb9000;
mod level1;
mod levels;
mod nomino_consts;
mod nominos;
mod piece_movement;
mod window_management;
mod window_utils;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugin(WindowManager);
    app.add_plugin(GroceryBagger9000Plugin);
    app.add_plugins(DefaultPlugins);
    app.add_plugin(ShapePlugin);

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        use bevy_screen_diags::ScreenDiagsPlugin;

        app.add_plugin(ScreenDiagsPlugin);
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(LogDiagnosticsPlugin::default());
    }

    app.run();
}
