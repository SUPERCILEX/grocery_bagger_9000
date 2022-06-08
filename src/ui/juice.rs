use crate::{
    animations::{score_particle, score_particle_fade_out, GameSpeed},
    bags::BagSize,
    levels::{LevelMarker, ScoreChanged, ScoringSystems},
    ui::{
        consts::{HUD_FONT_SIZE, SCORE_COLOR},
        PRIMARY_FONT,
    },
    window_management::{DipsWindow, WindowSystems},
};
use bevy::{prelude::*};


pub struct JuicePlugin;

impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            score_juice_handler
                .after(WindowSystems)
                .after(ScoringSystems),
        );
    }
}

fn score_juice_handler(
    mut commands: Commands,
    mut score_changes: EventReader<ScoreChanged>,
    game_speed: Res<GameSpeed>,
    dips_window: Res<DipsWindow>,
    asset_server: Res<AssetServer>,
    positions: Query<(&GlobalTransform, Option<&BagSize>)>,
) {
    let font = asset_server.load(PRIMARY_FONT);
    for ScoreChanged { diff, cause } in score_changes.iter() {
        let (position, bag_size) = positions.get(*cause).unwrap();

        let score_text = if *diff >= 0 {
            format!("+{}", diff)
        } else {
            format!("{}", diff)
        };
        let from = {
            let mut from = *position;
            from.scale = Vec3::splat(dips_window.scale);
            from.translation.z = 100.;
            if let Some(bag_size) = bag_size {
                from.translation.y += bag_size.half_height();
            }
            from
        };
        let to = {
            let mut to = from;
            to.translation.y += 3.;
            to
        };
        let from_color = SCORE_COLOR;
        let to_color = {
            let mut color = from_color;
            color.set_a(0.);
            color
        };

        commands
            .spawn_bundle(Text2dBundle {
                transform: from.into(),
                text: Text::with_section(
                    score_text,
                    TextStyle {
                        font: font.clone(),
                        font_size: HUD_FONT_SIZE,
                        color: from_color,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Bottom,
                    },
                ),
                ..default()
            })
            .insert(LevelMarker)
            .insert(score_particle(from, to, &game_speed))
            .insert(score_particle_fade_out(from_color, to_color, &game_speed));
    }
}
