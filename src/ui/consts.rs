use bevy::prelude::Color;

use crate::hex_color;

pub const HUD_FONT_SIZE: f32 = 32.;
pub const IN_GAME_MENU_FONT_SIZE: f32 = 20.;
pub const TITLE_FONT_SIZE: f32 = 48.;
pub const MENU_FONT_SIZE: f32 = 32.;

pub const BUTTON_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = hex_color!(101, 118, 205);
pub const HOVERED_BUTTON: Color = hex_color!(138, 161, 238);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const SCORE_COLOR: Color = hex_color!(221, 111, 46);
pub const TITLE_COLOR: Color = BUTTON_COLOR;
