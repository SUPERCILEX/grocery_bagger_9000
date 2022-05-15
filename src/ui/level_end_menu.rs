use bevy::{app::Plugin, prelude::*, ui::PositionType::Absolute};

use crate::{
    levels::{CurrentLevel, GameState::Playing, LevelFinishedEvent, LevelTransitionLabel},
    App,
};

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
    // _score: Res<CurrentScore>,
    // _level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
    mut menu: ResMut<LevelEndMenu>,
) {
    if level_end.iter().count() == 0 {
        return;
    }
    // TODO: put playing update into callback for button
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                position_type: Absolute,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
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
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..default()
                    });
                });
        })
        .id();
    menu.root = Some(root);
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut level: ResMut<CurrentLevel>,
    mut text_query: Query<&mut Text>,
    menu: ResMut<LevelEndMenu>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                level.state = Playing;
                level.level += 1;
                commands.entity(menu.root.unwrap()).despawn_recursive();
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
