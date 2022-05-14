use std::fmt::Write;

use bevy::prelude::*;

use crate::levels::scoring::CurrentScore;

pub struct DisplayScorePlugin;

impl Plugin for DisplayScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score);
        app.add_system(update_score);
    }
}

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::BLUE;

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
}
