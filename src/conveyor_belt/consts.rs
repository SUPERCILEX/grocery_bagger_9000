use bevy::prelude::Color;

use crate::hex_color;

pub const MAX_NUM_PIECES: u8 = 9;
pub const PIECE_WIDTH: f32 = 3.;
pub const HEIGHT: f32 = PIECE_WIDTH * 2.;
pub const LENGTH: f32 = PIECE_WIDTH * MAX_NUM_PIECES as f32 + 2.5 * SELECTABLE_SEPARATION;

pub const SELECTABLE_SEPARATION: f32 = 2.;
pub const NON_SELECTABLE_LIGHTNESS: f32 = 0.38;

pub const BELT_SELECTABLE_BACKGROUND_COLOR: Color = hex_color!(0x83, 0x39, 0x9B);
pub const BELT_NONSELECTABLE_BACKGROUND_COLOR: Color = hex_color!(43, 21, 54);
