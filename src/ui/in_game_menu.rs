use bevy::{
    prelude::*,
    ui::{PositionType::Absolute, UiSystem},
};

use crate::{
    levels::{LevelFinished, LevelMarker, LevelSpawnStage},
    run_criteria::run_if_level_started,
    ui::consts::{BUTTON_COLOR, IN_GAME_MENU_FONT_SIZE, NORMAL_BUTTON},
};

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            LevelSpawnStage,
            setup_menu.with_run_criteria(run_if_level_started),
        );
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            handle_restart_level_click.after(UiSystem::Focus),
        );
    }
}

#[derive(Component)]
struct RestartLevelButton;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(LevelMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        padding: Rect {
                            left: Val::Px(15.),
                            right: Val::Px(15.),
                            top: Val::Px(7.5),
                            bottom: Val::Px(7.5),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(RestartLevelButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Restart Level",
                            TextStyle {
                                font,
                                font_size: IN_GAME_MENU_FONT_SIZE,
                                color: BUTTON_COLOR,
                            },
                            default(),
                        ),
                        ..default()
                    });
                });
        });
}

fn handle_restart_level_click(
    mut level_finished: EventWriter<LevelFinished>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartLevelButton>)>,
) {
    if let Ok(interaction) = interaction_query.get_single() && *interaction == Interaction::Clicked {
        level_finished.send(LevelFinished);
    }
}
