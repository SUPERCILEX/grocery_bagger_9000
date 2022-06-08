use bevy::{prelude::*, ui::PositionType::Absolute};

use crate::{hex_color, levels::LevelMarker, ui::PRIMARY_FONT};

pub const TUTORIAL_FONT_SIZE_LARGE: f32 = 32.;
pub const TUTORIAL_FONT_SIZE_SMALL: f32 = 24.;
pub const TUTORIAL_FONT_COLOR: Color = hex_color!(138, 161, 238);

// TODO get rid of closure after https://github.com/rust-lang/rust/issues/67792
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
    let font = asset_server.load(PRIMARY_FONT);
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
