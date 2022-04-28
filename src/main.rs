#![feature(let_chains)]
#![feature(once_cell)]
#![allow(clippy::too_many_arguments)]

use bevy::{app::App, DefaultPlugins};
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::{gb9000::GroceryBagger9000Plugin, window_management::WindowManager};

mod gb9000;
mod nomino_consts;
mod nominos;
mod piece_movement;
mod window_management;
mod window_utils;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(WindowManager)
        .add_plugin(GroceryBagger9000Plugin);

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        use bevy_screen_diags::ScreenDiagsPlugin;

        app.add_plugin(ScreenDiagsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }

    app.run();
}
