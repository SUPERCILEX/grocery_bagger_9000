use bevy::prelude::Color;

pub const MAX_NUM_PIECES: usize = 9;
pub const PIECE_WIDTH: f32 = 3.;
pub const HEIGHT: f32 = PIECE_WIDTH * 2.;
pub const LENGTH: f32 = PIECE_WIDTH * MAX_NUM_PIECES as f32;

pub const SELECTABLE_SEPARATION: f32 = 2.;
pub const NON_SELECTABLE_LIGHTNESS: f32 = 0.38;

pub const BELT_SELECTABLE_BACKGROUND_COLOR: Color =
    Color::rgb(0xE0 as f32 / 255., 0xE0 as f32 / 255., 0xE0 as f32 / 255.);
pub const BELT_NONSELECTABLE_BACKGROUND_COLOR: Color =
    Color::rgb(0x9E as f32 / 255., 0x9E as f32 / 255., 0x9E as f32 / 255.);
