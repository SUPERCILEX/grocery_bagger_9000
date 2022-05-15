use bevy::{app::Plugin, prelude::*, ui::PositionType::Absolute};

use crate::{
    levels::{CurrentLevel, GameState::Playing, LevelFinishedEvent, LevelTransitionLabel},
    App,
};
use crate::levels::CurrentScore;
use crate::ui::display_score::{FONT_COLOR, FONT_SIZE};

const BUTTON_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub struct LevelEndMenuPlugin;

impl Plugin for LevelEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelEndMenu>();
        app.add_system_to_stage(
            CoreStage::Last,
            show_level_end_screen.after(LevelTransitionLabel),
        );
        app.add_system_to_stage(CoreStage::Last, button_system.after(show_level_end_screen));
    }
}

#[derive(Debug, Default)]
pub struct LevelEndMenu {
    root: Option<Entity>,
}

fn show_level_end_screen(
    mut commands: Commands,
    mut level_end: EventReader<LevelFinishedEvent>,
    score: Res<CurrentScore>,
    level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
    mut menu: ResMut<LevelEndMenu>,
) {
    if level_end.iter().count() == 0 {
        return;
    }

    // TODO: put playing update into callback for button
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // level text
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("Level: {}", level.level + 1),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE * 1.5,
                                color: Color::BLACK,
                            },
                        }],
                        ..Default::default()
                    },
                    style: Style{
                        margin: Rect{bottom: Val::Px(20.), ..default()},
                        ..default()
                    },
                    ..Default::default()
                });

            // score text
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("Score: {}", score.points),
                            style: TextStyle {
                                font,
                                font_size: FONT_SIZE,
                                color: FONT_COLOR,
                            },
                        }],
                        ..Default::default()
                    },
                    style: Style{
                        margin: Rect{bottom: Val::Px(40.), ..default()},
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
                        // margin: Rect{top: Val::Percent(10.), bottom: Val::Percent(10.), ..default()},
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
                                font_size: 40.0,
                                color: BUTTON_COLOR,
                            },
                            Default::default(),
                        ),
                        ..default()
                    });
                });
        }).id();
    menu.root = Some(root);
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut level: ResMut<CurrentLevel>,
    menu: ResMut<LevelEndMenu>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                level.state = Playing;
                level.level += 1;
                commands.entity(menu.root.unwrap()).despawn_recursive();
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
