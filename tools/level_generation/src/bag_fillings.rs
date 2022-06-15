use std::{collections::HashSet, iter::repeat};

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

    const fn blocks(self) -> &'static [(i8, i8)] {
        match self {
            Self::TrominoStraight => &[(0, 0), (0, 1), (0, 2)],
            Self::TrominoStraight180 => &[(0, 0), (1, 0), (2, 0)],
            Self::TrominoL => &[(0, 0), (0, -1), (1, 0)],
            Self::TrominoL90 => &[(0, 0), (1, 0), (0, 1)],
            Self::TrominoL180 => &[(0, 0), (1, 0), (0, -1)],
            Self::TrominoL270 => &[(0, 0), (-1, 0), (0, -1)],
            Self::TetrominoStraight => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::TetrominoStraight180 => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::TetrominoSquare => &[(0, 0), (0, 1), (1, 0), (1, 1)],
            Self::TetrominoT => &[(0, 0), (0, 2), (1, 1), (0, 1)],
            Self::TetrominoT90 => &[(0, 0), (2, 0), (1, -1), (1, 0)],
            Self::TetrominoT180 => &[(0, 0), (0, 2), (-1, 1), (0, 1)],
            Self::TetrominoT270 => &[(0, 0), (2, 0), (1, 1), (1, 0)],
            Self::TetrominoL => &[(0, 0), (2, 0), (1, 0), (0, 1)],
            Self::TetrominoL90 => &[(0, 0), (0, -2), (0, -1), (1, 0)],
            Self::TetrominoL180 => &[(0, 0), (-2, 0), (-1, 0), (0, -1)],
            Self::TetrominoL270 => &[(0, 0), (0, 2), (0, 1), (-1, 0)],
            Self::TetrominoLMirrored => &[(0, 0), (2, 0), (1, 0), (0, -1)],
            Self::TetrominoLMirrored90 => &[(0, 0), (0, -2), (0, -1), (-1, 0)],
            Self::TetrominoLMirrored180 => &[(0, 0), (-2, 0), (-1, 0), (0, 1)],
            Self::TetrominoLMirrored270 => &[(0, 0), (0, 2), (0, 1), (1, 0)],
            Self::TetrominoSkew => &[(0, 0), (1, 0), (1, 1), (2, 1)],
            Self::TetrominoSkew180 => &[(0, 0), (0, 1), (-1, 1), (-1, 2)],
            Self::TetrominoSkewMirrored => &[(0, 0), (1, 0), (1, -1), (2, -1)],
            Self::TetrominoSkewMirrored180 => &[(0, 0), (0, 1), (1, 1), (1, 2)],
        }
    }
}

#[derive(Debug, Default)]
struct Scratchpad {
    bag_width: u8,
    bag_height: u8,
    bag_matrix: Vec<Vec<u8>>,
    search_space: Vec<(RawNomino, u8, u8, u8)>,
    undo_ops: Vec<(usize, usize)>,
}

impl Scratchpad {
    fn new(bag_width: u8, bag_height: u8) -> Self {
        let mut bag_matrix = Vec::new();
        for _ in 0..bag_height {
            bag_matrix.push(repeat(0).take(usize::from(bag_width)).collect());
        }

        Self {
            bag_width,
            bag_height,
            bag_matrix,
            ..Self::default()
        }
    }

    fn extend_search_space(&mut self, depth: u8) {
        for (row_num, row) in self.bag_matrix.iter().enumerate() {
            for (col, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    for piece in PIECES {
                        self.search_space.push((
                            *piece,
                            depth,
                            u8::try_from(row_num).unwrap(),
                            u8::try_from(col).unwrap(),
                        ));
                    }
                }
            }
        }
    }

    fn erase_at_depth(&mut self, depth: u8) {
        for row in &mut self.bag_matrix {
            for cell in row {
                if *cell == depth {
                    *cell = 0;
                }
            }
        }
    }

    fn attempt_piece_placement(
        &mut self,
        blocks: &[(i8, i8)],
        depth: u8,
        target_row: u8,
        target_col: u8,
    ) -> bool {
        let mut failed = false;
        for (offset_row, offset_col) in blocks {
            let row = i16::from(target_row) + i16::from(*offset_row);
            let col = i16::from(target_col) + i16::from(*offset_col);
            if row < 0
                || row >= i16::from(self.bag_height)
                || col < 0
                || col >= i16::from(self.bag_width)
            {
                failed = true;
                break;
            }

            let row = usize::try_from(row).unwrap();
            let col = usize::try_from(col).unwrap();
            let cell = &mut self.bag_matrix[row][col];
            if *cell > 0 {
                failed = true;
                break;
            }

            *cell = depth + 1;
            self.undo_ops.push((row, col));
        }

        if failed {
            while let Some((row, col)) = self.undo_ops.pop() {
                self.bag_matrix[row][col] = 0;
            }
            false
        } else {
            self.undo_ops.clear();
            true
        }
    }
}

pub fn generate(width: u8, height: u8) -> HashSet<Vec<Nomino>> {
    let mut bags = HashSet::new();
    let mut piece_stack = Vec::with_capacity(8);

    let mut scratchpad = Scratchpad::new(width, height);

    scratchpad.extend_search_space(0);
    while let Some((piece, depth, target_row, target_col)) = scratchpad.search_space.pop() {
        while piece_stack.len() > usize::from(depth) {
            scratchpad.erase_at_depth(u8::try_from(piece_stack.len()).unwrap());
            piece_stack.pop();
        }

        let blocks = piece.blocks();
        let succeeded = scratchpad.attempt_piece_placement(blocks, depth, target_row, target_col);

        if !succeeded {
            continue;
        }

        let block_count = u8::try_from(blocks.len()).unwrap();
        let block_count = if let Some((_, last_count)) = piece_stack.last() {
            last_count + block_count
        } else {
            block_count
        };
        piece_stack.push((piece, block_count));

        if block_count == width * height {
            let mut bag = piece_stack
                .iter()
                .map(|(p, _)| p.into_nomino())
                .collect::<Vec<_>>();
            bag.sort_unstable();
            bags.insert(bag);
        } else {
            scratchpad.extend_search_space(depth + 1);
        }
    }

    bags
}
