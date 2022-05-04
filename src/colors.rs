use bevy::prelude::*;

#[derive(Copy, Clone, Component)]
pub enum NominoColor {
    Orange,
    _Last,
    Debug,
}

impl NominoColor {
    pub const COUNT: usize = NominoColor::_Last as usize;

    pub fn render(&self) -> Color {
        match self {
            NominoColor::Orange => Color::ORANGE,
            NominoColor::Debug => Color::BLACK,
            NominoColor::_Last => unreachable!(),
        }
    }

    pub fn id(&self) -> usize {
        *self as usize
    }
}
