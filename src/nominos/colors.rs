use std::ops::{Index, IndexMut};

use bevy::prelude::*;

use crate::hex_color;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub enum NominoColor {
    Orange,
    Gold,
    Blue,
    Green,
    Pink,
    #[cfg(feature = "debug")]
    Debug,
    _Last,
}

impl NominoColor {
    pub const COUNT: usize = Self::_Last as usize;

    pub fn render(self) -> Color {
        match self {
            Self::Orange => hex_color!(0xCC, 0x65, 0x2D),
            Self::Gold => hex_color!(0xD6, 0xC5, 0x42),
            Self::Blue => hex_color!(0x04, 0xBD, 0xDE),
            Self::Green => hex_color!(0x6C, 0xE0, 0xB2),
            Self::Pink => hex_color!(0xC2, 0x2B, 0xA6),
            #[cfg(feature = "debug")]
            Self::Debug => Color::WHITE,
            Self::_Last => unreachable!(),
        }
    }
}

impl<T> Index<NominoColor> for [T] {
    type Output = T;

    fn index(&self, index: NominoColor) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<NominoColor> for [T] {
    fn index_mut(&mut self, index: NominoColor) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
