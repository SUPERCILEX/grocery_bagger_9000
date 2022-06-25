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

    const fn blocks(self) -> &'static [(usize, isize)] {
        match self {
            Self::TrominoStraight => &[(0, 1), (0, 2)],
            Self::TrominoStraight180 => &[(1, 0), (2, 0)],
            Self::TrominoL => &[(0, 1), (1, 1)],
            Self::TrominoL90 => &[(1, 0), (0, 1)],
            Self::TrominoL180 => &[(1, 0), (1, -1)],
            Self::TrominoL270 => &[(1, 0), (1, 1)],
            Self::TetrominoStraight => &[(0, 1), (0, 2), (0, 3)],
            Self::TetrominoStraight180 => &[(1, 0), (2, 0), (3, 0)],
            Self::TetrominoSquare => &[(0, 1), (1, 0), (1, 1)],
            Self::TetrominoT => &[(0, 1), (0, 2), (1, 1)],
            Self::TetrominoT90 => &[(1, 0), (2, 0), (1, -1)],
            Self::TetrominoT180 => &[(1, -1), (1, 0), (1, 1)],
            Self::TetrominoT270 => &[(1, 0), (2, 0), (1, 1)],
            Self::TetrominoL => &[(1, 0), (2, 0), (0, 1)],
            Self::TetrominoL90 => &[(0, 1), (0, 2), (1, 2)],
            Self::TetrominoL180 => &[(1, 0), (2, 0), (2, -1)],
            Self::TetrominoL270 => &[(1, 0), (1, 1), (1, 2)],
            Self::TetrominoLMirrored => &[(0, 1), (1, 1), (2, 1)],
            Self::TetrominoLMirrored90 => &[(1, -2), (1, -1), (1, 0)],
            Self::TetrominoLMirrored180 => &[(1, 0), (2, 0), (2, 1)],
            Self::TetrominoLMirrored270 => &[(0, 1), (0, 2), (1, 0)],
            Self::TetrominoSkew => &[(1, 0), (1, 1), (2, 1)],
            Self::TetrominoSkew180 => &[(1, -1), (1, 0), (0, 1)],
            Self::TetrominoSkewMirrored => &[(1, 0), (1, -1), (2, -1)],
            Self::TetrominoSkewMirrored180 => &[(0, 1), (1, 1), (1, 2)],
        }
    }
}

#[derive(Debug, Default)]
struct Scratchpad {
    bag_width: usize,
    bag_height: usize,
    full_count: usize,
    bag_matrix: Box<[Box<[u8]>]>,
    search_space: Vec<(RawNomino, u8, usize, usize)>,
    undo_ops: Vec<(usize, usize)>,
    scratch_bag: Box<[Box<[u8]>]>,
}

impl Scratchpad {
    fn new(bag_width: usize, bag_height: usize) -> Self {
        let mut bag_matrix = Vec::new();
        for _ in 0..bag_height {
            bag_matrix.push(repeat(0).take(bag_width).collect());
        }

        Self {
            bag_width,
            bag_height,
            full_count: bag_width * bag_height,
            bag_matrix: bag_matrix.into(),
            ..Self::default()
        }
    }

    fn extend_search_space(&mut self, depth: u8, block_count: usize) {
        self.scratch_bag.clone_from(&self.bag_matrix);

        for (row_num, row) in self.bag_matrix.iter().enumerate() {
            for (col, cell) in row.iter().enumerate() {
                if *cell > 0 {
                    continue;
                }

                for piece in PIECES {
                    Self::apply_pending_undo_ops_disjoint(
                        &mut self.scratch_bag,
                        &mut self.undo_ops,
                    );

                    let succeeded = Self::attempt_piece_placement_disjoint(
                        self.bag_width,
                        self.bag_height,
                        &mut self.scratch_bag,
                        &mut self.undo_ops,
                        piece.blocks(),
                        depth,
                        row_num,
                        col,
                    );

                    let failed = !succeeded
                        || Self::is_duplicate_disjoint(&self.scratch_bag, &self.undo_ops);
                    if failed {
                        continue;
                    }

                    let block_count_diff =
                        self.full_count - (block_count + piece.blocks().len() + 1);
                    if block_count_diff == 5 || (block_count_diff < 3 && block_count_diff != 0) {
                        continue;
                    }

                    self.search_space.push((*piece, depth, row_num, col));
                }
                Self::apply_pending_undo_ops_disjoint(&mut self.scratch_bag, &mut self.undo_ops);
            }
        }
    }

    fn erase_at_depth(&mut self, depth: u8) {
        for row in &mut *self.bag_matrix {
            for cell in &mut **row {
                if *cell == depth {
                    *cell = 0;
                }
            }
        }
    }

    fn place_piece(
        &mut self,
        blocks: &[(usize, isize)],
        depth: u8,
        target_row: usize,
        target_col: usize,
    ) {
        self.bag_matrix[target_row][target_col] = depth + 1;
        for (offset_row, offset_col) in blocks {
            let row = target_row + *offset_row;
            let col = usize::try_from(isize::try_from(target_col).unwrap() + *offset_col).unwrap();

            self.bag_matrix[row][col] = depth + 1;
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn attempt_piece_placement_disjoint(
        bag_width: usize,
        bag_height: usize,
        bag_matrix: &mut [Box<[u8]>],
        undo_ops: &mut Vec<(usize, usize)>,
        blocks: &[(usize, isize)],
        depth: u8,
        target_row: usize,
        target_col: usize,
    ) -> bool {
        for (offset_row, offset_col) in blocks {
            let row = target_row + *offset_row;
            let col = isize::try_from(target_col).unwrap() + *offset_col;
            if row >= bag_height || col < 0 {
                return false;
            }
            let col = usize::try_from(col).unwrap();
            if col >= bag_width {
                return false;
            }

            let cell = &mut bag_matrix[row][col];
            if *cell > 0 {
                return false;
            }

            *cell = depth + 1;
            undo_ops.push((row, col));
        }

        bag_matrix[target_row][target_col] = depth + 1;
        undo_ops.push((target_row, target_col));

        true
    }

    fn apply_pending_undo_ops_disjoint(
        bag_matrix: &mut [Box<[u8]>],
        undo_ops: &mut Vec<(usize, usize)>,
    ) {
        while let Some((row, col)) = undo_ops.pop() {
            bag_matrix[row][col] = 0;
        }
    }

    fn is_duplicate_disjoint(bag_matrix: &[Box<[u8]>], undo_ops: &Vec<(usize, usize)>) -> bool {
        let mut valid = false;
        for (row, col) in undo_ops {
            if *row > 0 && bag_matrix[*row - 1][*col] == 0 {
                return true;
            }
            valid |= *col == 0 || bag_matrix[*row][*col - 1] > 0;
        }
        !valid
    }
}

pub fn generate(width: usize, height: usize) -> HashSet<Vec<Nomino>> {
    let mut bags = HashSet::new();
    let mut piece_stack = Vec::with_capacity(8);

    let mut scratchpad = Scratchpad::new(width, height);

    scratchpad.extend_search_space(0, 0);
    while let Some((piece, depth, target_row, target_col)) = scratchpad.search_space.pop() {
        while piece_stack.len() > usize::from(depth) {
            scratchpad.erase_at_depth(u8::try_from(piece_stack.len()).unwrap());
            piece_stack.pop();
        }

        let blocks = piece.blocks();
        scratchpad.place_piece(blocks, depth, target_row, target_col);

        let block_count = blocks.len() + 1;
        let block_count = if let Some((_, last_count)) = piece_stack.last() {
            last_count + block_count
        } else {
            block_count
        };
        piece_stack.push((piece, block_count));

        if block_count == scratchpad.full_count {
            let mut bag = piece_stack
                .iter()
                .map(|(p, _)| p.into_nomino())
                .collect::<Vec<_>>();
            bag.sort_unstable();
            bags.insert(bag);
        } else {
            scratchpad.extend_search_space(depth + 1, block_count);
        }
    }

    bags
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Write};

    use goldenfile::Mint;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn bag_fillings(#[values(3, 4, 5)] width: usize, #[values(3, 4, 5)] height: usize) {
        let mut mint = Mint::new("testdata/bag_fillings");
        let file = mint.new_goldenfile(format!("{width}x{height}")).unwrap();
        let mut writer = BufWriter::new(file);

        let bags = generate(width, height);
        let mut bags = bags.iter().collect::<Vec<_>>();
        bags.sort_unstable();
        for bag in bags {
            writeln!(writer, "{:?}", bag).unwrap();
        }
        writer.flush().unwrap();
    }
}
