use bevy::{app::Plugin, prelude::*, ui::PositionType::Absolute};

use crate::{
    animations,
    animations::GameSpeed,
    gb9000::{
        GameState::{LevelEnded, Playing},
        GroceryBagger9000,
    },
    levels::{
        CurrentScore, LevelFinished, LevelStarted, LevelTransitionSystems, ScoringSystems,
        LAST_LEVEL,
    },
    ui::consts::{BUTTON_COLOR, MENU_FONT_SIZE, NORMAL_BUTTON, TITLE_FONT_SIZE},
    App,
};

pub struct LevelEndMenuPlugin;

impl Plugin for LevelEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            show_level_end_screen
                .after(LevelTransitionSystems)
                .after(ScoringSystems),
        );
        app.add_system(despawn_menu.after(LevelTransitionSystems));

        app.add_system(handle_restart_level_click.before(LevelTransitionSystems));
        app.add_system(handle_next_level_click.before(LevelTransitionSystems));
    }
}

#[derive(Component)]
struct MenuMarker;

#[derive(Component)]
struct RestartLevelButton;

#[derive(Component)]
struct NextLevelButton;

fn show_level_end_screen(
    mut commands: Commands,
    mut level_end: EventReader<LevelFinished>,
    score: Res<CurrentScore>,
    game_speed: Res<GameSpeed>,
    gb9000: Res<GroceryBagger9000>,
    asset_server: Res<AssetServer>,
) {
    if level_end.iter().count() == 0 || gb9000.state != LevelEnded {
        return;
    }

    let from = Rect {
        bottom: Val::Percent(100.),
        ..default()
    };
    let to = Rect {
        bottom: Val::Percent(0.),
        ..default()
    };

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position: from,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(MenuMarker)
        .insert(animations::level_complete_menu_ui_enter(
            from,
            to,
            &game_speed,
        ))
        .with_children(|parent| {
            spawn_level_completed_summary(parent, &gb9000, font.clone());
            spawn_score_recap(parent, &score, font.clone());
            spawn_restart_and_next_level_buttons(parent, &gb9000, font);
        });
}

fn spawn_level_completed_summary(
    parent: &mut ChildBuilder,
    gb9000: &GroceryBagger9000,
    font: Handle<Font>,
) {
    parent.spawn_bundle(TextBundle {
        text: Text::with_section(
            if gb9000.current_level == LAST_LEVEL {
                "Game complete!".to_string()
            } else {
                format!("Level {} complete", gb9000.current_level + 1)
            },
            TextStyle {
                font,
                font_size: TITLE_FONT_SIZE,
                color: Color::BLACK,
            },
            default(),
        ),
        style: Style {
            margin: Rect {
                bottom: Val::Px(20.),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

fn spawn_score_recap(parent: &mut ChildBuilder, score: &CurrentScore, font: Handle<Font>) {
    parent.spawn_bundle(TextBundle {
        text: Text::with_section(
            format!(
                "Score: {}\nAll time score: {}",
                score.points, score.all_time_points
            ),
            TextStyle {
                font,
                font_size: MENU_FONT_SIZE,
                color: Color::BLUE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..default()
            },
        ),
        style: Style {
            margin: Rect {
                bottom: Val::Px(40.),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

fn spawn_restart_and_next_level_buttons(
    parent: &mut ChildBuilder,
    gb9000: &GroceryBagger9000,
    font: Handle<Font>,
) {
    let button_bundle = ButtonBundle {
        style: Style {
            padding: Rect {
                left: Val::Px(30.),
                right: Val::Px(30.),
                top: Val::Px(15.),
                bottom: Val::Px(15.),
            },
            margin: Rect {
                left: Val::Px(10.),
                right: Val::Px(10.),
                ..default()
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    };

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(button_bundle.clone())
                .insert(RestartLevelButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Restart Level",
                            TextStyle {
                                font: font.clone(),
                                font_size: MENU_FONT_SIZE,
                                color: BUTTON_COLOR,
                            },
                            default(),
                        ),
                        ..default()
                    });
                });

            parent
                .spawn_bundle(button_bundle)
                .insert(NextLevelButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            if gb9000.current_level == LAST_LEVEL {
                                "Start infinite mode"
                            } else {
                                "Next Level"
                            },
                            TextStyle {
                                font,
                                font_size: MENU_FONT_SIZE,
                                color: BUTTON_COLOR,
                            },
                            default(),
                        ),
                        ..default()
                    });
                });
        });
}

fn handle_next_level_click(
    mut gb9000: ResMut<GroceryBagger9000>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NextLevelButton>)>,
) {
    if let Ok(interaction) = interaction_query.get_single() && *interaction == Interaction::Clicked {
        gb9000.state = Playing;
        gb9000.current_level += 1;
    }
}

fn handle_restart_level_click(
    mut gb9000: ResMut<GroceryBagger9000>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartLevelButton>)>,
) {
    if let Ok(interaction) = interaction_query.get_single() && *interaction == Interaction::Clicked {
        gb9000.state = Playing;
    }
}

fn despawn_menu(
    mut commands: Commands,
    mut level_started: EventReader<LevelStarted>,
    mut prev_level: Local<u16>,
    menus: Query<Entity, With<MenuMarker>>,
    game_speed: Res<GameSpeed>,
) {
    let restarted = if let Some(started) = level_started.iter().last() {
        let restarted = *prev_level == **started;
        *prev_level = **started;
        restarted
    } else {
        return;
    };

    let (from, to) = if restarted {
        (
            Rect {
                bottom: Val::Percent(0.),
                ..default()
            },
            Rect {
                bottom: Val::Percent(100.),
                ..default()
            },
        )
    } else {
        (
            Rect {
                top: Val::Percent(0.),
                ..default()
            },
            Rect {
                top: Val::Percent(100.),
                ..default()
            },
        )
    };

    for menu in menus.iter() {
        let animator = animations::level_complete_menu_ui_exit(from, to, &game_speed);
        commands.entity(menu).insert(animator);
    }
}
