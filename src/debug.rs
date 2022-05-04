use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.init_resource::<DebugOptions>();
        app.add_system(debug_options);
    }
}

#[derive(Default)]
pub struct DebugOptions {
    pub unrestricted_pieces: bool,
}

fn debug_options(mut egui_context: ResMut<EguiContext>, mut debug_options: ResMut<DebugOptions>) {
    egui::Window::new("Debug options")
        .open(&mut true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.checkbox(
                &mut debug_options.unrestricted_pieces,
                "Allow unrestricted piece movement",
            )
        });
}
