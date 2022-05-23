use bevy::{app::Plugin, prelude::*, ui::PositionType::Absolute};

use crate::{
    animations,
    animations::GameSpeed,
    gb9000::{GameState::Playing, GroceryBagger9000},
    levels::{CurrentScore, LevelFinished, LevelStarted, LevelTransitionSystems, ScoringSystems},
    ui::consts::{MENU_FONT_SIZE, TITLE_FONT_SIZE},
    App,
};

const BUTTON_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub struct LevelEndMenuPlugin;

impl Plugin for LevelEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_menu.after(LevelTransitionSystems));
        app.add_system(
            show_level_end_screen
                .after(LevelTransitionSystems)
                .after(ScoringSystems),
        );
        app.add_system(button_system.before(LevelTransitionSystems));
    }
}

#[derive(Component)]
struct MenuMarker;

fn show_level_end_screen(
    mut commands: Commands,
    mut level_end: EventReader<LevelFinished>,
    score: Res<CurrentScore>,
    game_speed: Res<GameSpeed>,
    gb9000: ResMut<GroceryBagger9000>,
    asset_server: Res<AssetServer>,
) {
    if level_end.iter().count() == 0 {
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
            // level text
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("Level {} complete", gb9000.current_level + 1),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: TITLE_FONT_SIZE,
                            color: Color::BLACK,
                        },
                    }],
                    ..Default::default()
                },
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(20.),
                        ..default()
                    },
                    ..default()
                },
                ..Default::default()
            });

            // score text
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("Score: {}", score.points),
                        style: TextStyle {
                            font,
                            font_size: MENU_FONT_SIZE,
                            color: Color::BLUE,
                        },
                    }],
                    ..Default::default()
                },
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(40.),
                        ..default()
                    },
                    ..default()
                },
                ..Default::default()
            });

            // next level button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Next Level",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: MENU_FONT_SIZE,
                                color: BUTTON_COLOR,
                            },
                            Default::default(),
                        ),
                        ..default()
                    });
                });
        });
}

fn button_system(
    mut gb9000: ResMut<GroceryBagger9000>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                gb9000.state = Playing;
                gb9000.current_level += 1;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn despawn_menu(
    mut commands: Commands,
    mut level_started: EventReader<LevelStarted>,
    menus: Query<Entity, With<MenuMarker>>,
    game_speed: Res<GameSpeed>,
) {
    if level_started.iter().count() == 0 {
        return;
    }

    let from = Rect {
        top: Val::Percent(0.),
        ..default()
    };
    let to = Rect {
        top: Val::Percent(100.),
        ..default()
    };
    for menu in menus.iter() {
        let animator = animations::level_complete_menu_ui_exit(from, to, &game_speed);
        commands.entity(menu).insert(animator);
    }
}
