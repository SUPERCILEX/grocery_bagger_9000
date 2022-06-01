use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSystem};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_screen_diags::ScreenDiagsPlugin;

use crate::{
    animations::GameSpeed,
    colors::NominoColor,
    conveyor_belt::ConveyorBeltOptions,
    gb9000::{
        GameState::{LevelEnded, Playing},
        GroceryBagger9000,
    },
    levels::{LevelFinished, LevelMarker},
    nominos::*,
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
        app.add_plugin(ScreenDiagsPlugin);

        app.init_resource::<DebugOptions>();

        app.add_system_to_stage(
            CoreStage::PreUpdate,
            debug_options.after(EguiSystem::BeginFrame),
        );
        app.add_system(open_debug_menu);
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
    Straight3,
    L3,
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
    const fn name(&self) -> &str {
        match self {
            Self::Straight3 => "Straight (3)",
            Self::L3 => "L (3)",
            Self::Straight => "Straight",
            Self::Square => "Square",
            Self::T => "T",
            Self::L => "L",
            Self::L2 => "Mirrored L",
            Self::Skew => "Skew",
            Self::Skew2 => "Mirrored Skew",
        }
    }
}

#[allow(clippy::too_many_lines)]
fn debug_options(
    mut egui_context: ResMut<EguiContext>,
    mut debug_options: ResMut<DebugOptions>,
    mut inspector: ResMut<WorldInspectorParams>,
    mut nomino_to_spawn: Local<NominoType>,
    mut nomino_color_to_spawn: Local<NominoColorWrapper>,
    mut commands: Commands,
    mut gb9000: ResMut<GroceryBagger9000>,
    mut conveyor_belt_options: ResMut<ConveyorBeltOptions>,
    mut game_speed: ResMut<GameSpeed>,
    mut level_finished: EventWriter<LevelFinished>,
) {
    let debug_options = &mut *debug_options;
    egui::Window::new("Debug options")
        .open(&mut debug_options.open)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                let mut level_num = gb9000.current_level;

                ui.label("Level");
                ui.add(
                    egui::DragValue::new(&mut level_num)
                        .clamp_range(1..=u16::MAX)
                        .speed(0.025),
                );

                if level_num != gb9000.current_level {
                    gb9000.current_level = level_num;
                    gb9000.state = Playing;
                    level_finished.send(LevelFinished);
                }

                if gb9000.state != LevelEnded && ui.button("Finish").clicked() {
                    gb9000.state = LevelEnded;
                    level_finished.send(LevelFinished);
                }
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
                    debug_options.unrestricted_pieces = true;

                    commands
                        .spawn_bundle(TransformBundle::default())
                        .insert(LevelMarker)
                        .with_children(|parent| {
                            let position = Transform::from_xyz(3., 3., 0.);

                            macro_rules! spawn {
                                ($nomino:expr) => {{
                                    spawn!($nomino, Transform::default())
                                }};

                                ($nomino:expr, $transform:expr) => {{
                                    parent
                                        .spawn_nomino_into_bag(
                                            position,
                                            $nomino,
                                            **nomino_color_to_spawn,
                                            $transform,
                                        )
                                        .insert(Selectable)
                                }};
                            }

                            match *nomino_to_spawn {
                                NominoType::Straight3 => spawn!(Nomino::TrominoStraight),
                                NominoType::L3 => spawn!(Nomino::TrominoL),
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
                        option!(NominoColor::Gold);
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

                        option!(NominoType::Straight3);
                        option!(NominoType::L3);
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
