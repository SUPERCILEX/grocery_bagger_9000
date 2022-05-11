use bevy::prelude::*;
use rand::{
    distributions::{Distribution, WeightedIndex},
    thread_rng, Rng,
};

use crate::{
    colors::NominoColor,
    nominos::{Nomino, DEG_180, DEG_MIRRORED},
};

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

pub struct RandomPiecesConveyorBelt<const NOM_TYPES: usize, const COLORS: usize> {
    num_pieces: usize,
    nomino_types: [Nomino; NOM_TYPES],
    colors: [NominoColor; COLORS],
    color_dist: WeightedIndex<usize>,
}

impl<const NOM_TYPES: usize, const COLORS: usize> RandomPiecesConveyorBelt<NOM_TYPES, COLORS> {
    pub fn new(
        num_pieces: usize,
        nomino_types: [Nomino; NOM_TYPES],
        colors: [NominoColor; COLORS],
    ) -> Self {
        Self::with_color_dist(num_pieces, nomino_types, colors, [1; COLORS])
    }

    pub fn with_color_dist(
        num_pieces: usize,
        nomino_types: [Nomino; NOM_TYPES],
        colors: [NominoColor; COLORS],
        color_weights: [usize; COLORS],
    ) -> Self {
        debug_assert!(NOM_TYPES > 0 && NOM_TYPES <= Nomino::_Last as usize);
        debug_assert!(COLORS > 0 && COLORS <= NominoColor::_Last as usize);

        Self {
            num_pieces,
            nomino_types,
            colors,
            color_dist: WeightedIndex::new(color_weights).unwrap(),
        }
    }
}

impl<const NOM_TYPES: usize, const COLORS: usize> ConveyorBelt
    for RandomPiecesConveyorBelt<NOM_TYPES, COLORS>
{
    fn next(&mut self) -> Option<Piece> {
        if self.num_pieces == 0 {
            return None;
        }
        self.num_pieces -= 1;

        let mut rng = thread_rng();
        let next_shape = rng.gen_range(0..NOM_TYPES);
        let next_color = self.color_dist.sample(&mut rng);
        let mut rotation = if rng.gen() { *DEG_MIRRORED } else { default() };
        if rng.gen() {
            rotation *= *DEG_180;
        }

        Some(Piece {
            nomino: self.nomino_types[next_shape],
            color: self.colors[next_color],
            rotation,
        })
    }
}
