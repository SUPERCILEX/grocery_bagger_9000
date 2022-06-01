use bevy::prelude::Color;

pub const MAX_NUM_PIECES: u8 = 9;
pub const PIECE_WIDTH: f32 = 3.;
pub const HEIGHT: f32 = PIECE_WIDTH * 2.;
pub const LENGTH: f32 = PIECE_WIDTH * MAX_NUM_PIECES as f32 + 2.5 * SELECTABLE_SEPARATION;

pub const SELECTABLE_SEPARATION: f32 = 2.;
pub const NON_SELECTABLE_LIGHTNESS: f32 = 0.38;

pub const BELT_SELECTABLE_BACKGROUND_COLOR: Color = /* Color::rgb(
        0xE0u8 as f32 / 255.,
        0xE0u8 as f32 / 255.,
        0xE0u8 as f32 / 255.,
    ); */
    Color::rgb(155. / 255., 59. / 255., 180. / 255.);

pub const BELT_NONSELECTABLE_BACKGROUND_COLOR: Color = /* Color::rgb(
        0x9Eu8 as f32 / 255.,
        0x9Eu8 as f32 / 255.,
        0x9Eu8 as f32 / 255.,
    ); */
    Color::rgb(43. / 255., 21. / 255., 54. / 255.);
