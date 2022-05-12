use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_screen_diags::ScreenDiagsPlugin;

use crate::{
    animations::GameSpeed, colors::NominoColor, conveyor_belt::ConveyorBeltOptions,
    levels::CurrentLevel, nominos::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.insert_resource(WorldInspectorParams {
            enabled: false,
            highlight_changes: true,
            despawnable_entities: true,
            ..default()
        });
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(InspectableRapierPlugin);
        app.init_resource::<DebugOptions>();

        app.add_system(debug_options);
        app.add_system(open_debug_menu);

        app.add_plugin(ScreenDiagsPlugin);
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}

pub struct DebugOptions {
    pub unrestricted_pieces: bool,
    open: bool,
}

impl Default for DebugOptions {
    fn default() -> Self {
        Self {
            unrestricted_pieces: false,
            open: true,
        }
    }
}

#[derive(Default, Eq, PartialEq)]
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

#[derive(Deref, DerefMut)]
struct NominoColorWrapper(NominoColor);

impl Default for NominoColorWrapper {
    fn default() -> Self {
        Self(NominoColor::Debug)
    }
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
    mut inspector: ResMut<WorldInspectorParams>,
    mut nomino_to_spawn: Local<NominoType>,
    mut nomino_color_to_spawn: Local<NominoColorWrapper>,
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    mut conveyor_belt_options: ResMut<ConveyorBeltOptions>,
    mut game_speed: ResMut<GameSpeed>,
) {
    let debug_options = &mut *debug_options;
    egui::Window::new("Debug options")
        .open(&mut debug_options.open)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Level");
                ui.add(egui::DragValue::new(&mut current_level.level).speed(0.025));
            });

            ui.horizontal(|ui| {
                ui.label("Game speed");
                ui.add(
                    egui::DragValue::new(&mut **game_speed)
                        .speed(0.01)
                        .clamp_range(0.01..=100.),
                );
            });

            ui.checkbox(
                &mut debug_options.unrestricted_pieces,
                "Allow unrestricted piece movement",
            );

            ui.horizontal(|ui| {
                ui.label("Number of selectable conveyor belt pieces");
                ui.add(
                    egui::DragValue::new(&mut conveyor_belt_options.num_pieces_selectable)
                        .speed(0.025)
                        .clamp_range(0..=9),
                );
            });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Spawn").clicked() {
                    if let Some(root) = current_level.root {
                        commands.entity(root).with_children(|parent| {
                            let position = Transform::from_xyz(3., 3., 0.);

                            macro_rules! spawn {
                                ($nomino:expr) => {{
                                    spawn!($nomino, Transform::default())
                                }};

                                ($nomino:expr, $transform:expr) => {{
                                    parent
                                        .spawn_nomino(
                                            position,
                                            $nomino,
                                            **nomino_color_to_spawn,
                                            $transform,
                                        )
                                        .insert(Selectable)
                                }};
                            }

                            match *nomino_to_spawn {
                                NominoType::Straight => spawn!(Nomino::TetrominoStraight),
                                NominoType::Square => spawn!(Nomino::TetrominoSquare),
                                NominoType::T => spawn!(Nomino::TetrominoT),
                                NominoType::L => spawn!(Nomino::TetrominoL),
                                NominoType::L2 => {
                                    spawn!(
                                        Nomino::TetrominoL,
                                        Transform::from_rotation(*DEG_MIRRORED)
                                    )
                                }
                                NominoType::Skew => spawn!(Nomino::TetrominoSkew),
                                NominoType::Skew2 => {
                                    spawn!(
                                        Nomino::TetrominoSkew,
                                        Transform::from_rotation(*DEG_MIRRORED)
                                    )
                                }
                            };
                        });
                    }
                }

                egui::ComboBox::from_id_source("Nomino color to spawn")
                    .selected_text(format!("{:?}", **nomino_color_to_spawn))
                    .show_ui(ui, |ui| {
                        macro_rules! option {
                            ($nomino:expr) => {
                                ui.selectable_value(
                                    &mut **nomino_color_to_spawn,
                                    $nomino,
                                    format!("{:?}", $nomino),
                                );
                            };
                        }

                        option!(NominoColor::Red);
                        option!(NominoColor::Orange);
                        option!(NominoColor::Blue);
                        option!(NominoColor::Green);
                        option!(NominoColor::Pink);
                        option!(NominoColor::Debug);
                    });

                egui::ComboBox::from_id_source("Nomino to spawn")
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
            });

            if ui.button("Open inspector").clicked() {
                inspector.enabled = true;
            }
        });
}

fn open_debug_menu(keys: Res<Input<KeyCode>>, mut debug_options: ResMut<DebugOptions>) {
    if keys.just_pressed(KeyCode::Semicolon) {
        debug_options.open = true;
    }
}
