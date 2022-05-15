use std::fmt::Write;

use bevy::{prelude::*, ui::PositionType::Absolute};

use crate::{
    levels::{CurrentScore, LevelFinishedEvent, LevelLoaded},
    ui::consts::HUD_FONT_SIZE,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_hud);
        app.add_system(update_score);
        app.add_system(despawn_huds);
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct ScoreText;

fn setup_hud(
    mut commands: Commands,
    mut level_loaded: EventReader<LevelLoaded>,
    asset_server: Res<AssetServer>,
) {
    if level_loaded.iter().count() == 0 {
        return;
    }

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
        .insert(Hud)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: String::new(),
                            style: TextStyle {
                                font,
                                font_size: HUD_FONT_SIZE,
                                color: Color::BLUE,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}

fn update_score(score: Res<CurrentScore>, mut text_query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }

    if let Ok(mut text) = text_query.get_single_mut() {
        let text = &mut text.sections[0].value;

        text.clear();
        write!(text, "Score: {}", score.points).unwrap();
    }
}

fn despawn_huds(
    mut commands: Commands,
    mut level_finished: EventReader<LevelFinishedEvent>,
    huds: Query<Entity, With<Hud>>,
) {
    if level_finished.iter().count() == 0 {
        return;
    }

    for hud in huds.iter() {
        commands.entity(hud).despawn_recursive();
    }
}