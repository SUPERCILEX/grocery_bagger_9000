#![feature(let_chains)]
#![feature(once_cell)]
#![feature(option_result_contains)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::{app::App, DefaultPlugins};
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::{gb9000::GroceryBagger9000Plugin, window_management::WindowManager};

mod bag_replacement;
mod bags;
mod colors;
mod conveyor_belt;
mod conveyor_belt_movement;
#[cfg(feature = "debug")]
mod debug;
mod gb9000;
mod level1;
mod level2;
mod level3;
mod level4;
mod level5;
mod levels;
mod nomino_consts;
mod nominos;
mod piece_movement;
mod scoring;
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

    #[cfg(feature = "debug")]
    app.add_plugin(debug::DebugPlugin);

    app.run();
}
