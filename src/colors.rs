use std::ops::{Index, IndexMut};

use bevy::prelude::*;

#[derive(Copy, Clone, Component)]
pub enum NominoColor {
    Red,
    Orange,
    Blue,
    Green,
    Pink,
    #[cfg(feature = "debug")]
    Debug,
    _Last,
}

impl NominoColor {
    pub const COUNT: usize = NominoColor::_Last as usize;

    pub fn render(&self) -> Color {
        match self {
            NominoColor::Red => Color::RED,
            NominoColor::Orange => Color::ORANGE,
            NominoColor::Blue => Color::CYAN,
            NominoColor::Green => Color::GREEN,
            NominoColor::Pink => Color::FUCHSIA,
            #[cfg(feature = "debug")]
            NominoColor::Debug => Color::WHITE,
            NominoColor::_Last => unreachable!(),
        }
    }

    pub fn id(&self) -> usize {
        *self as usize
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
