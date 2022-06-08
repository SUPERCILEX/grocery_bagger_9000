use std::fmt::Write;

use bevy::{ecs::schedule::ShouldRun, prelude::*, ui::PositionType::Absolute};
use num_format::{Locale, ToFormattedString};

use crate::{
    levels::{CurrentScore, LevelMarker, LevelSpawnStage, ScoringSystems},
    run_criteria::run_if_level_started,
    ui::consts::{HUD_FONT_SIZE, SCORE_COLOR},
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            update_score
                .with_run_criteria(run_if_score_changed)
                .after(ScoringSystems),
        );

        app.add_system_to_stage(
            LevelSpawnStage,
            setup_hud.with_run_criteria(run_if_level_started),
        );
    }
}

#[derive(Component)]
struct ScoreText;

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(LevelMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Score: 0",
                        TextStyle {
                            font,
                            font_size: HUD_FONT_SIZE,
                            color: SCORE_COLOR,
                        },
                        default(),
                    ),
                    ..default()
                })
                .insert(ScoreText);
        });
}

fn run_if_score_changed(score: Res<CurrentScore>) -> ShouldRun {
    if score.is_changed() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn update_score(score: Res<CurrentScore>, mut text_query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = text_query.get_single_mut() {
        let text = &mut text.sections[0].value;

        text.clear();
        write!(
            text,
            "Score: {}",
            score.points.to_formatted_string(&Locale::en)
        )
        .unwrap();
    }
}
