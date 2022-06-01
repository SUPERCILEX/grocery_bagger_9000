use std::ops::{Index, IndexMut};

use bevy::prelude::*;

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
        // match self {
        //     Self::Red => Color::RED,
        //     Self::Gold => Color::GOLD,
        //     Self::Blue => Color::CYAN,
        //     Self::Green => Color::GREEN,
        //     Self::Pink => Color::FUCHSIA,
        //     #[cfg(feature = "debug")]
        //     Self::Debug => Color::WHITE,
        //     Self::_Last => unreachable!(),
        // }
        match self {
            Self::Red => Color::hex("DD6F2E").unwrap(),
            Self::Gold => Color::hex("F1DE4A").unwrap(),
            Self::Blue => Color::hex("68E2FC").unwrap(),
            Self::Green => Color::hex("76FBC6").unwrap(),
            Self::Pink => Color::hex("DC2FBB").unwrap(),
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
