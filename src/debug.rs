use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::{levels::CurrentLevel, markers::Selectable, nomino_consts::DEG_MIRRORED, nominos::*};

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

#[derive(Default, Debug, PartialEq)]
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

fn debug_options(
    mut egui_context: ResMut<EguiContext>,
    mut debug_options: ResMut<DebugOptions>,
    mut nomino_to_spawn: Local<NominoType>,
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
) {
    egui::Window::new("Debug options")
        .open(&mut true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.checkbox(
                &mut debug_options.unrestricted_pieces,
                "Allow unrestricted piece movement",
            );

            ui.group(|ui| {
                egui::ComboBox::from_label("Nomino to spawn")
                    .selected_text(format!("{:?}", *nomino_to_spawn))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut *nomino_to_spawn,
                            NominoType::Straight,
                            "Straight",
                        );
                        ui.selectable_value(&mut *nomino_to_spawn, NominoType::Square, "Square");
                        ui.selectable_value(&mut *nomino_to_spawn, NominoType::T, "T");
                        ui.selectable_value(&mut *nomino_to_spawn, NominoType::L, "L");
                        ui.selectable_value(&mut *nomino_to_spawn, NominoType::L2, "Mirrored L");
                        ui.selectable_value(&mut *nomino_to_spawn, NominoType::Skew, "Skew");
                        ui.selectable_value(
                            &mut *nomino_to_spawn,
                            NominoType::Skew2,
                            "Mirrored Skew",
                        );
                    });

                if ui.button("Spawn nomino").clicked() {
                    if let Some(root) = current_level.root {
                        commands.entity(root).with_children(|parent| {
                            let position = Transform::from_xyz(3., 3., 0.);
                            match *nomino_to_spawn {
                                NominoType::Straight => parent
                                    .spawn_nomino(
                                        position,
                                        TetrominoStraight,
                                        Color::BLACK,
                                        Transform::default(),
                                    )
                                    .insert(Selectable),
                                NominoType::Square => parent
                                    .spawn_nomino(
                                        position,
                                        TetrominoSquare,
                                        Color::BLACK,
                                        Transform::default(),
                                    )
                                    .insert(Selectable),
                                NominoType::T => parent
                                    .spawn_nomino(
                                        position,
                                        TetrominoT,
                                        Color::BLACK,
                                        Transform::default(),
                                    )
                                    .insert(Selectable),
                                NominoType::L => parent
                                    .spawn_nomino(
                                        position,
                                        TetrominoL,
                                        Color::BLACK,
                                        Transform::default(),
                                    )
                                    .insert(Selectable),
                                NominoType::L2 => parent
                                    .spawn_nomino(
                                        position,
                                        TetrominoL,
                                        Color::BLACK,
                                        Transform::from_rotation(*DEG_MIRRORED),
                                    )
                                    .insert(Selectable),
                                NominoType::Skew => parent
                                    .spawn_nomino(
                                        position,
                                        TetrominoSkew,
                                        Color::BLACK,
                                        Transform::default(),
                                    )
                                    .insert(Selectable),
                                NominoType::Skew2 => parent
                                    .spawn_nomino(
                                        position,
                                        TetrominoSkew,
                                        Color::BLACK,
                                        Transform::from_rotation(*DEG_MIRRORED),
                                    )
                                    .insert(Selectable),
                            };
                        });
                    }
                }
            });
        });
}
