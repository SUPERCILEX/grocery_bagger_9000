use bevy::{prelude::*, ui::PositionType::Absolute};

use crate::levels::LevelMarker;

pub const TUTORIAL_FONT_SIZE_LARGE: f32 = 32.;
pub const TUTORIAL_FONT_SIZE_SMALL: f32 = 24.;
pub const TUTORIAL_FONT_COLOR: Color = Color::rgb(138. / 255., 161. / 255., 238. / 255.); //Color::rgb(222. / 255., 156. / 255., 69. / 255.); //hex("DE9C45").unwrap();

pub const TUTORIAL_STYLE: fn() -> Style = || Style {
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    position_type: Absolute,
    flex_direction: FlexDirection::ColumnReverse,
    align_items: AlignItems::FlexStart,
    justify_content: JustifyContent::FlexStart,
    position: Rect {
        top: Val::Percent(33.),
        left: Val::Percent(5.),
        ..default()
    },
    ..default()
};

pub fn spawn_text_tutorial(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    text: impl Into<String>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: TUTORIAL_STYLE(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(LevelMarker)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font,
                        font_size: TUTORIAL_FONT_SIZE_SMALL,
                        color: TUTORIAL_FONT_COLOR,
                    },
                    default(),
                ),
                ..default()
            });
        });
}
