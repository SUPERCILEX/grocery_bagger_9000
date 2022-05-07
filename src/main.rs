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

    #[cfg(feature = "dump")]
    {
        use std::{fs, fs::File, io::Write, process::Command};

        use bevy::render::{render_graph::RenderGraph, RenderApp, RenderStage};
        use bevy_mod_debugdump::{render_graph::render_graph_dot, schedule_graph::*};

        app.update();

        File::create("schedule.dot")
            .unwrap()
            .write_all(schedule_graph_dot(&app).as_bytes())
            .unwrap();
        File::create("render.dot")
            .unwrap()
            .write_all({
                let render_app = app.get_sub_app(RenderApp).unwrap();
                let render_graph = render_app.world.get_resource::<RenderGraph>().unwrap();

                render_graph_dot(&*render_graph).as_bytes()
            })
            .unwrap();
        File::create("render_schedule.dot")
            .unwrap()
            .write_all(
                schedule_graph_dot_sub_app_styled(
                    &app,
                    RenderApp,
                    &[&RenderStage::Extract],
                    &ScheduleGraphStyle::default(),
                )
                .as_bytes(),
            )
            .unwrap();

        for f in ["schedule", "render", "render_schedule"] {
            let svg = Command::new("sh")
                .arg("-c")
                .arg(format!("cat {f}.dot | dot -Tsvg"))
                .output()
                .unwrap();

            File::create(format!("{f}.svg"))
                .unwrap()
                .write_all(&svg.stdout)
                .unwrap();
            fs::remove_file(format!("{f}.dot")).unwrap();
        }
    }

    #[cfg(not(feature = "dump"))]
    app.run();
}
