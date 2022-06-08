use std::{collections::HashSet, iter::repeat};

use smallvec::SmallVec;

const PIECES: &[RawNomino] = &[
    RawNomino::TrominoStraight,
    RawNomino::TrominoStraight180,
    RawNomino::TrominoL,
    RawNomino::TrominoL90,
    RawNomino::TrominoL180,
    RawNomino::TrominoL270,
    RawNomino::TetrominoStraight,
    RawNomino::TetrominoStraight180,
    RawNomino::TetrominoSquare,
    RawNomino::TetrominoT,
    RawNomino::TetrominoT90,
    RawNomino::TetrominoT180,
    RawNomino::TetrominoT270,
    RawNomino::TetrominoL,
    RawNomino::TetrominoL90,
    RawNomino::TetrominoL180,
    RawNomino::TetrominoL270,
    RawNomino::TetrominoLMirrored,
    RawNomino::TetrominoLMirrored90,
    RawNomino::TetrominoLMirrored180,
    RawNomino::TetrominoLMirrored270,
    RawNomino::TetrominoSkew,
    RawNomino::TetrominoSkew180,
    RawNomino::TetrominoSkewMirrored,
    RawNomino::TetrominoSkewMirrored180,
];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Nomino {
    TrominoStraight,
    TrominoL,
    TetrominoStraight,
    TetrominoSquare,
    TetrominoT,
    TetrominoL,
    TetrominoLMirrored,
    TetrominoSkew,
    TetrominoSkewMirrored,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum RawNomino {
    TrominoStraight,
    TrominoStraight180,
    TrominoL,
    TrominoL90,
    TrominoL180,
    TrominoL270,
    TetrominoStraight,
    TetrominoStraight180,
    TetrominoSquare,
    TetrominoT,
    TetrominoT90,
    TetrominoT180,
    TetrominoT270,
    TetrominoL,
    TetrominoL90,
    TetrominoL180,
    TetrominoL270,
    TetrominoLMirrored,
    TetrominoLMirrored90,
    TetrominoLMirrored180,
    TetrominoLMirrored270,
    TetrominoSkew,
    TetrominoSkew180,
    TetrominoSkewMirrored,
    TetrominoSkewMirrored180,
}

impl RawNomino {
    const fn into_nomino(self) -> Nomino {
        match self {
            Self::TrominoStraight | Self::TrominoStraight180 => Nomino::TrominoStraight,
            Self::TrominoL | Self::TrominoL90 | Self::TrominoL180 | Self::TrominoL270 => {
                Nomino::TrominoL
            }
            Self::TetrominoStraight | Self::TetrominoStraight180 => Nomino::TetrominoStraight,
            Self::TetrominoSquare => Nomino::TetrominoSquare,
            Self::TetrominoT | Self::TetrominoT90 | Self::TetrominoT180 | Self::TetrominoT270 => {
                Nomino::TetrominoT
            }
            Self::TetrominoL | Self::TetrominoL90 | Self::TetrominoL180 | Self::TetrominoL270 => {
                Nomino::TetrominoL
            }
            Self::TetrominoLMirrored
            | Self::TetrominoLMirrored90
            | Self::TetrominoLMirrored180
            | Self::TetrominoLMirrored270 => Nomino::TetrominoLMirrored,
            Self::TetrominoSkew | Self::TetrominoSkew180 => Nomino::TetrominoSkew,
            Self::TetrominoSkewMirrored | Self::TetrominoSkewMirrored180 => {
                Nomino::TetrominoSkewMirrored
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn blocks(self) -> SmallVec<[(i8, i8); 4]> {
        let mut blocks = SmallVec::new();
        blocks.push((0, 0));
        match self {
            Self::TrominoStraight => {
                blocks.push((0, 1));
                blocks.push((0, 2));
            }
            Self::TrominoStraight180 => {
                blocks.push((1, 0));
                blocks.push((2, 0));
            }
            Self::TrominoL => {
                blocks.push((0, -1));
                blocks.push((1, 0));
            }
            Self::TrominoL90 => {
                blocks.push((1, 0));
                blocks.push((0, 1));
            }
            Self::TrominoL180 => {
                blocks.push((1, 0));
                blocks.push((0, -1));
            }
            Self::TrominoL270 => {
                blocks.push((-1, 0));
                blocks.push((0, -1));
            }
            Self::TetrominoStraight => {
                blocks.push((0, 1));
                blocks.push((0, 2));
                blocks.push((0, 3));
            }
            Self::TetrominoStraight180 => {
                blocks.push((1, 0));
                blocks.push((2, 0));
                blocks.push((3, 0));
            }
            Self::TetrominoSquare => {
                blocks.push((0, 1));
                blocks.push((1, 0));
                blocks.push((1, 1));
            }
            Self::TetrominoT => {
                blocks.push((0, 2));
                blocks.push((1, 1));
                blocks.push((0, 1));
            }
            Self::TetrominoT90 => {
                blocks.push((2, 0));
                blocks.push((1, -1));
                blocks.push((1, 0));
            }
            Self::TetrominoT180 => {
                blocks.push((0, 2));
                blocks.push((-1, 1));
                blocks.push((0, 1));
            }
            Self::TetrominoT270 => {
                blocks.push((2, 0));
                blocks.push((1, 1));
                blocks.push((1, 0));
            }
            Self::TetrominoL => {
                blocks.push((2, 0));
                blocks.push((1, 0));
                blocks.push((0, 1));
            }
            Self::TetrominoL90 => {
                blocks.push((0, -2));
                blocks.push((0, -1));
                blocks.push((1, 0));
            }
            Self::TetrominoL180 => {
                blocks.push((-2, 0));
                blocks.push((-1, 0));
                blocks.push((0, -1));
            }
            Self::TetrominoL270 => {
                blocks.push((0, 2));
                blocks.push((0, 1));
                blocks.push((-1, 0));
            }
            Self::TetrominoLMirrored => {
                blocks.push((2, 0));
                blocks.push((1, 0));
                blocks.push((0, -1));
            }
            Self::TetrominoLMirrored90 => {
                blocks.push((0, -2));
                blocks.push((0, -1));
                blocks.push((-1, 0));
            }
            Self::TetrominoLMirrored180 => {
                blocks.push((-2, 0));
                blocks.push((-1, 0));
                blocks.push((0, 1));
            }
            Self::TetrominoLMirrored270 => {
                blocks.push((0, 2));
                blocks.push((0, 1));
                blocks.push((1, 0));
            }
            Self::TetrominoSkew => {
                blocks.push((1, 0));
                blocks.push((1, 1));
                blocks.push((2, 1));
            }
            Self::TetrominoSkew180 => {
                blocks.push((0, 1));
                blocks.push((-1, 1));
                blocks.push((-1, 2));
            }
            Self::TetrominoSkewMirrored => {
                blocks.push((1, 0));
                blocks.push((1, -1));
                blocks.push((2, -1));
            }
            Self::TetrominoSkewMirrored180 => {
                blocks.push((0, 1));
                blocks.push((1, 1));
                blocks.push((1, 2));
            }
        }
        blocks
    }
}

struct Bag {
    blocks: SmallVec<[SmallVec<[u8; 6]>; 6]>,
    width: u8,
    height: u8,
}

impl Bag {
    fn new(width: u8, height: u8) -> Self {
        let mut blocks = SmallVec::new();
        for _ in 0..height {
            blocks.push(repeat(0).take(usize::from(width)).collect());
        }

        Self {
            blocks,
            width,
            height,
        }
    }

    fn extend_search_space(&self, search_space: &mut Vec<(RawNomino, u8, u8, u8)>, depth: u8) {
        for piece in PIECES {
            for row in 0..self.height {
                for col in 0..self.width {
                    search_space.push((*piece, depth, row, col));
                }
            }
        }
    }

    fn erase_at_depth(&mut self, depth: u8) {
        for row in &mut self.blocks {
            for cell in row {
                if *cell == depth {
                    *cell = 0;
                }
            }
        }
    }
}

pub fn generate(width: u8, height: u8) -> HashSet<SmallVec<[Nomino; 8]>> {
    let mut bags = HashSet::new();
    let mut piece_stack = SmallVec::<[_; 16]>::new();
    let mut search_space =
        Vec::with_capacity(PIECES.len() * usize::from(width) * usize::from(height));

    let mut scratchpad = Bag::new(width, height);

    scratchpad.extend_search_space(&mut search_space, 0);
    'outer: while let Some((piece, depth, target_row, target_col)) = search_space.pop() {
        while piece_stack.len() > usize::from(depth) {
            scratchpad.erase_at_depth(u8::try_from(piece_stack.len()).unwrap());
            piece_stack.pop();
        }

        let blocks = piece.blocks();
        let block_count = u8::try_from(blocks.len()).unwrap();
        let block_count = if let Some((_, last_count)) = piece_stack.last() {
            last_count + block_count
        } else {
            block_count
        };
        piece_stack.push((piece, block_count));

        for (offset_row, offset_col) in blocks {
            let row = i16::from(target_row) + i16::from(offset_row);
            let col = i16::from(target_col) + i16::from(offset_col);
            if row < 0 || row >= i16::from(height) || col < 0 || col >= i16::from(width) {
                continue 'outer;
            }

            let row = usize::try_from(row).unwrap();
            let col = usize::try_from(col).unwrap();
            let cell = &mut scratchpad.blocks[row][col];
            if *cell > 0 {
                continue 'outer;
            }

            *cell = u8::try_from(piece_stack.len()).unwrap();
        }

        if block_count == width * height {
            let mut bag = piece_stack
                .iter()
                .map(|(p, _)| p.into_nomino())
                .collect::<SmallVec<_>>();
            bag.sort_unstable();
            bags.insert(bag);
        } else {
            scratchpad.extend_search_space(&mut search_space, depth + 1);
        }
    }

    bags
}
