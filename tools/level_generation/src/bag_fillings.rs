use std::{collections::HashSet, iter::repeat, thread};

use serde::Serialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize)]
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
    search_space: Vec<(RawNomino, u8, (usize, usize))>,
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
        let mut target_row = 0;
        let mut target_col = 0;
        'outer: for (row_num, row) in self.bag_matrix.iter().enumerate() {
            for (col, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    target_row = row_num;
                    target_col = col;
                    break 'outer;
                }
            }
        }

        macro_rules! attempt_placement {
            ($piece:expr) => {
                let piece = $piece;
                if self.attempt_piece_placement(piece.blocks(), target_row, target_col) {
                    let block_count_diff =
                        self.full_count - (block_count + piece.blocks().len() + 1);
                    if block_count_diff != 5 && !(block_count_diff < 3 && block_count_diff != 0) {
                        self.search_space
                            .push((piece, depth, (target_row, target_col)));
                    }
                }
            };
        }

        attempt_placement!(RawNomino::TrominoStraight);
        attempt_placement!(RawNomino::TrominoStraight180);
        attempt_placement!(RawNomino::TrominoL);
        attempt_placement!(RawNomino::TrominoL90);
        attempt_placement!(RawNomino::TrominoL180);
        attempt_placement!(RawNomino::TrominoL270);
        attempt_placement!(RawNomino::TetrominoStraight);
        attempt_placement!(RawNomino::TetrominoStraight180);
        attempt_placement!(RawNomino::TetrominoSquare);
        attempt_placement!(RawNomino::TetrominoT);
        attempt_placement!(RawNomino::TetrominoT90);
        attempt_placement!(RawNomino::TetrominoT180);
        attempt_placement!(RawNomino::TetrominoT270);
        attempt_placement!(RawNomino::TetrominoL);
        attempt_placement!(RawNomino::TetrominoL90);
        attempt_placement!(RawNomino::TetrominoL180);
        attempt_placement!(RawNomino::TetrominoL270);
        attempt_placement!(RawNomino::TetrominoLMirrored);
        attempt_placement!(RawNomino::TetrominoLMirrored90);
        attempt_placement!(RawNomino::TetrominoLMirrored180);
        attempt_placement!(RawNomino::TetrominoLMirrored270);
        attempt_placement!(RawNomino::TetrominoSkew);
        attempt_placement!(RawNomino::TetrominoSkew180);
        attempt_placement!(RawNomino::TetrominoSkewMirrored);
        attempt_placement!(RawNomino::TetrominoSkewMirrored180);
    }

    fn place_piece(
        &mut self,
        blocks: &[(usize, isize)],
        depth: u8,
        target_row: usize,
        target_col: usize,
    ) {
        unsafe {
            *self
                .bag_matrix
                .get_unchecked_mut(target_row)
                .get_unchecked_mut(target_col) = depth;
        }
        for (offset_row, offset_col) in blocks {
            let row = target_row + *offset_row;
            unsafe {
                let col =
                    usize::try_from(isize::try_from(target_col).unwrap_unchecked() + *offset_col)
                        .unwrap_unchecked();

                *self
                    .bag_matrix
                    .get_unchecked_mut(row)
                    .get_unchecked_mut(col) = depth;
            }
        }
    }

    #[inline]
    fn attempt_piece_placement(
        &self,
        blocks: &[(usize, isize)],
        target_row: usize,
        target_col: usize,
    ) -> bool {
        for (offset_row, offset_col) in blocks {
            let row = target_row + *offset_row;
            let col = unsafe { isize::try_from(target_col).unwrap_unchecked() } + *offset_col;
            if row >= self.bag_height || col < 0 {
                return false;
            }
            let col = unsafe { usize::try_from(col).unwrap_unchecked() };
            if col >= self.bag_width {
                return false;
            }

            let cell = unsafe { self.bag_matrix.get_unchecked(row).get_unchecked(col) };
            if *cell > 0 {
                return false;
            }
        }
        true
    }
}

pub fn generate(width: usize, height: usize) -> HashSet<Vec<Nomino>> {
    let mut bags = HashSet::new();

    let seed_search_space = {
        let mut scratchpad = Scratchpad::new(width, height);
        scratchpad.extend_search_space(0, 0);
        scratchpad.search_space
    };

    thread::scope(|scope| {
        let mut sub_problems = Vec::with_capacity(seed_search_space.len());

        for seed in seed_search_space {
            let mut scratchpad = Scratchpad::new(width, height);
            scratchpad.search_space.push(seed);
            sub_problems.push(scope.spawn(move || exhaust_scratchpad(scratchpad)));
        }

        for problem in sub_problems {
            bags.extend(problem.join().unwrap());
        }
    });

    bags
}

fn exhaust_scratchpad(mut scratchpad: Scratchpad) -> HashSet<Vec<Nomino>, ahash::RandomState> {
    let mut bags = HashSet::with_hasher(ahash::RandomState::new());
    let mut piece_stack = Vec::<(RawNomino, usize, (usize, usize))>::with_capacity(8);
    let mut completed_bag = Vec::new();

    while let Some((piece, depth, (target_row, target_col))) = scratchpad.search_space.pop() {
        while piece_stack.len() > usize::from(depth) {
            let (piece, _, (target_row, target_col)) = piece_stack.pop().unwrap();
            scratchpad.place_piece(piece.blocks(), 0, target_row, target_col);
        }

        let blocks = piece.blocks();
        scratchpad.place_piece(blocks, depth + 1, target_row, target_col);

        let block_count = blocks.len() + 1;
        let block_count = if let Some((_, last_count, _)) = piece_stack.last() {
            last_count + block_count
        } else {
            block_count
        };
        piece_stack.push((piece, block_count, (target_row, target_col)));

        if block_count == scratchpad.full_count {
            completed_bag.extend(piece_stack.iter().map(|p| p.0.into_nomino()));
            completed_bag.sort_unstable();

            if !bags.contains(&completed_bag) {
                bags.insert(completed_bag.clone());
            }
            completed_bag.clear();
        } else {
            scratchpad.extend_search_space(depth + 1, block_count);
        }
    }

    bags
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use goldenfile::Mint;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[ignore]
    fn bag_fillings(#[values(3, 4, 5, 6)] width: usize, #[values(3, 4, 5, 6)] height: usize) {
        let mut mint = Mint::new("testdata/bag_fillings");
        let file = mint.new_goldenfile(format!("{width}x{height}")).unwrap();
        let writer = BufWriter::new(file);

        let bags = generate(width, height);
        let mut bags = bags.iter().collect::<Vec<_>>();
        bags.sort_unstable();
        serde_json::to_writer_pretty(writer, &bags).unwrap();
    }
}
