#![feature(const_fn_floating_point_arithmetic)]
#![feature(let_chains)]
#![feature(once_cell)]
#![feature(option_result_contains)]
#![feature(is_sorted)]
#![feature(let_else)]
#![feature(div_duration)]
#![feature(is_some_with)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::forget_non_drop)] // TODO https://github.com/bevyengine/bevy/issues/4601

use bevy::{app::App, DefaultPlugins};

use crate::{gb9000::GroceryBagger9000Plugin, window_management::WindowManager};

mod analytics;
mod animations;
mod bags;
mod colors;
mod conveyor_belt;
#[cfg(feature = "debug")]
mod debug;
mod gb9000;
mod levels;
mod nominos;
mod robot;
mod run_criteria;
mod ui;
mod window_management;
mod window_utils;

fn main() {
    let mut app = App::new();

    app.add_plugin(WindowManager);
    app.add_plugins(DefaultPlugins);
    app.add_plugin(GroceryBagger9000Plugin);

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

                render_graph_dot(render_graph).as_bytes()
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

    #[cfg(feature = "system-ambiguity")]
    {
        app.init_resource::<bevy::ecs::schedule::ReportExecutionOrderAmbiguities>();
        app.update();
    }

    #[cfg(all(not(feature = "dump"), not(feature = "system-ambiguity")))]
    app.run();
}
