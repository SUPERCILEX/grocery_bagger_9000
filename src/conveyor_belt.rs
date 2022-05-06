use bevy::prelude::*;

use crate::{colors::NominoColor, nominos::Nomino};

pub const MAX_NUM_PIECES: usize = 9;
pub const PIECE_WIDTH: f32 = 3.;
pub const HEIGHT: f32 = PIECE_WIDTH * 2.;
pub const LENGTH: f32 = PIECE_WIDTH * MAX_NUM_PIECES as f32;

#[derive(Default, Deref, DerefMut)]
pub struct ConveyorBeltInstance(pub Option<Box<dyn ConveyorBelt + Send + Sync>>);

pub trait ConveyorBelt {
    fn next(&mut self) -> Option<Piece>;
}

#[derive(Copy, Clone)]
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
    pub fn new(pieces: [Piece; N]) -> PresetPiecesConveyorBelt<N> {
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
