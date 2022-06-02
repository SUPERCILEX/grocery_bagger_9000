use bevy::prelude::*;
use num_traits::FromPrimitive;
use rand::{thread_rng, Rng};

use crate::nominos::{Nomino, NominoColor, DEG_180, DEG_MIRRORED};

pub trait ConveyorBelt {
    fn next(&mut self) -> Option<Piece>;
}

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Piece {
    pub nomino: Nomino,
    pub color: NominoColor,
    pub rotation: Quat,
}

pub struct PresetPiecesConveyorBelt<const N: usize> {
    pieces: [Piece; N],
    next: usize,
}

impl<const N: usize> PresetPiecesConveyorBelt<N> {
    pub const fn new(pieces: [Piece; N]) -> Self {
        Self { pieces, next: 0 }
    }
}

impl<const N: usize> ConveyorBelt for PresetPiecesConveyorBelt<N> {
    fn next(&mut self) -> Option<Piece> {
        if self.next >= N {
            return None;
        }

        let piece = self.pieces[self.next];
        self.next += 1;
        Some(piece)
    }
}

pub struct InfinitePiecesConveyorBelt<const COLORS: usize> {
    colors: [NominoColor; COLORS],
}

impl<const COLORS: usize> InfinitePiecesConveyorBelt<COLORS> {
    pub const fn new(colors: [NominoColor; COLORS]) -> Self {
        Self { colors }
    }
}

impl<const COLORS: usize> ConveyorBelt for InfinitePiecesConveyorBelt<COLORS> {
    fn next(&mut self) -> Option<Piece> {
        let mut rng = thread_rng();
        let next_shape = rng.gen_range(0..Nomino::_Last as usize);
        let next_color = rng.gen_range(0..COLORS);
        let mut rotation = if rng.gen() { *DEG_MIRRORED } else { default() };
        if rng.gen() {
            rotation *= *DEG_180;
        }

        Some(Piece {
            nomino: Nomino::from_usize(next_shape).unwrap(),
            color: self.colors[next_color],
            rotation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_preset_pieces_returns_none() {
        let mut belt = PresetPiecesConveyorBelt::new([]);

        assert_eq!(belt.next(), None);
    }

    #[test]
    fn preset_pieces_returns_them() {
        let pieces = [
            Piece {
                nomino: Nomino::TetrominoSkew,
                color: NominoColor::Green,
                rotation: Quat::from_xyzw(1., 2., 3., 4.),
            },
            Piece {
                nomino: Nomino::TetrominoL,
                color: NominoColor::Pink,
                rotation: Quat::from_xyzw(2., 1., 3., 4.),
            },
            Piece {
                nomino: Nomino::TetrominoSquare,
                color: NominoColor::Blue,
                rotation: Quat::from_xyzw(1., 2., 4., 3.),
            },
        ];
        let mut belt = PresetPiecesConveyorBelt::new(pieces);

        assert_eq!(belt.next(), Some(pieces[0]));
        assert_eq!(belt.next(), Some(pieces[1]));
        assert_eq!(belt.next(), Some(pieces[2]));
        assert_eq!(belt.next(), None);
    }
}
