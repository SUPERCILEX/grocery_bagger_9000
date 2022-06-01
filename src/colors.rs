use std::ops::{Index, IndexMut};

use bevy::prelude::*;

#[macro_export]
macro_rules! hex_color {
    ($r:expr, $g:expr, $b:expr) => {{
        Color::rgb($r as f32 / 255., $g as f32 / 255., $b as f32 / 255.)
    }};
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub enum NominoColor {
    Red,
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
            Self::Red => hex_color!(0xDD, 0x6F, 0x2E),
            Self::Gold => hex_color!(0xF1, 0xDE, 0x4A),
            Self::Blue => hex_color!(0x68, 0xE2, 0xFC),
            Self::Green => hex_color!(0x76, 0xFB, 0xC6),
            Self::Pink => hex_color!(0xDC, 0x2F, 0xBB),
            #[cfg(feature = "debug")]
            Self::Debug => Color::WHITE,
            Self::_Last => unreachable!(),
        }
    }

    pub const fn id(self) -> usize {
        self as usize
    }
}

impl<T> Index<NominoColor> for [T] {
    type Output = T;

    fn index(&self, index: NominoColor) -> &Self::Output {
        &self[index.id()]
    }
}

impl<T> IndexMut<NominoColor> for [T] {
    fn index_mut(&mut self, index: NominoColor) -> &mut Self::Output {
        &mut self[index.id()]
    }
}
