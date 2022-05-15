use std::fmt::Write;

use bevy::{prelude::*, ui::PositionType::Absolute};

use crate::levels::CurrentScore;

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::BLUE;

pub struct DisplayScorePlugin;

impl Plugin for DisplayScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score);
        app.add_system(update_score);
    }
}

#[derive(Component)]
struct ScoreText;

fn update_score(score: Res<CurrentScore>, mut text_query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }

    let new_score = score.points;
    let mut text = text_query.single_mut();
    let text = &mut text.sections[0].value;
    text.clear();

    write!(text, "Score: {new_score}").unwrap();
}

fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: String::new(),
                            style: TextStyle {
                                font,
                                font_size: FONT_SIZE,
                                color: FONT_COLOR,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}
