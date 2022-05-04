use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_screen_diags::ScreenDiagsPlugin;

use crate::{levels::CurrentLevel, markers::Selectable, nomino_consts::DEG_MIRRORED, nominos::*};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.init_resource::<DebugOptions>();
        app.add_system(debug_options);

        app.add_plugin(ScreenDiagsPlugin);
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}

#[derive(Default)]
pub struct DebugOptions {
    pub unrestricted_pieces: bool,
}

#[derive(Default, PartialEq)]
enum NominoType {
    #[default]
    Straight,
    Square,
    T,
    L,
    L2,
    Skew,
    Skew2,
}

impl NominoType {
    fn name(&self) -> &str {
        match self {
            NominoType::Straight => "Straight",
            NominoType::Square => "Square",
            NominoType::T => "T",
            NominoType::L => "L",
            NominoType::L2 => "Mirrored L",
            NominoType::Skew => "Skew",
            NominoType::Skew2 => "Mirrored Skew",
        }
    }
}

fn debug_options(
    mut egui_context: ResMut<EguiContext>,
    mut debug_options: ResMut<DebugOptions>,
    mut nomino_to_spawn: Local<NominoType>,
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
) {
    egui::Window::new("Debug options")
        .open(&mut true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add(egui::DragValue::new(&mut current_level.level).speed(0.025));

            ui.checkbox(
                &mut debug_options.unrestricted_pieces,
                "Allow unrestricted piece movement",
            );

            ui.group(|ui| {
                egui::ComboBox::from_label("Nomino to spawn")
                    .selected_text(nomino_to_spawn.name())
                    .show_ui(ui, |ui| {
                        macro_rules! option {
                            ($nomino:expr) => {
                                ui.selectable_value(&mut *nomino_to_spawn, $nomino, $nomino.name());
                            };
                        }

                        option!(NominoType::Straight);
                        option!(NominoType::Square);
                        option!(NominoType::T);
                        option!(NominoType::L);
                        option!(NominoType::L2);
                        option!(NominoType::Skew);
                        option!(NominoType::Skew2);
                    });

                if ui.button("Spawn nomino").clicked() {
                    if let Some(root) = current_level.root {
                        commands.entity(root).with_children(|parent| {
                            let position = Transform::from_xyz(3., 3., 0.);

                            macro_rules! spawn {
                                ($nomino:expr) => {{
                                    spawn!($nomino, Transform::default())
                                }};

                                ($nomino:expr, $transform:expr) => {{
                                    parent
                                        .spawn_nomino(position, $nomino, Color::BLACK, $transform)
                                        .insert(Selectable)
                                }};
                            }

                            match *nomino_to_spawn {
                                NominoType::Straight => spawn!(TetrominoStraight),
                                NominoType::Square => spawn!(TetrominoSquare),
                                NominoType::T => spawn!(TetrominoT),
                                NominoType::L => spawn!(TetrominoL),
                                NominoType::L2 => {
                                    spawn!(TetrominoL, Transform::from_rotation(*DEG_MIRRORED))
                                }
                                NominoType::Skew => spawn!(TetrominoSkew),
                                NominoType::Skew2 => {
                                    spawn!(TetrominoSkew, Transform::from_rotation(*DEG_MIRRORED))
                                }
                            };
                        });
                    }
                }
            });
        });
}
