use std::collections::{HashMap, HashSet, VecDeque};

use bevy::{math::const_vec3, prelude::*};
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    bags::{
        BagFilled, BagMarker, BagReplacementSystems, BagSize, LARGEST_BAG_CAPACITY,
        LARGEST_BAG_HEIGHT, LARGEST_BAG_WIDTH,
    },
    colors::NominoColor,
    levels::{LevelSpawnStage, LevelStarted},
    nominos::{NominoMarker, PiecePlaced, PieceSystems, NOMINO_COLLIDER_GROUP},
};

pub struct ScoringPlugin;

const BLOCK_POINT_VALUE: u16 = 25;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentScore>();

        app.add_system(
            score_bags
                .label(ScoringSystems)
                .after(PieceSystems)
                .before(BagReplacementSystems),
        );
        app.add_system(reclaim_memory.after(BagReplacementSystems));
        app.add_system_to_stage(LevelSpawnStage, reset_score.label(ScoringSystems));
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub struct ScoringSystems;

#[derive(Debug, Default)]
pub struct CurrentScore {
    pub points: usize,
    pub all_time_points: usize,
    score_map: HashMap<Entity, u16>,
}

fn score_bags(
    mut piece_placements: EventReader<PiecePlaced>,
    bags: Query<(&GlobalTransform, &BagSize), With<BagMarker>>,
    color_wrapper: Query<&NominoColor, With<NominoMarker>>,
    rapier_context: Res<RapierContext>,
    mut current_score: ResMut<CurrentScore>,
) {
    for PiecePlaced { bag, .. } in piece_placements.iter() {
        let (bag_coords, bag_size) = bags.get(*bag).unwrap();

        let width = bag_size.width() as usize;
        let height = bag_size.height() as usize;
        let block_origin = bag_coords.translation - bag_size.origin() + const_vec3!([0.5, 0.5, 0.]);

        let mut bag_matrix = [[false; LARGEST_BAG_WIDTH]; LARGEST_BAG_HEIGHT];
        let mut block_count = 0u8;
        let mut color_block_count_map = [0u8; NominoColor::COUNT];
        for (row_num, row) in bag_matrix[..height].iter_mut().enumerate() {
            for (col, cell) in row[..width].iter_mut().enumerate() {
                rapier_context.intersections_with_point(
                    block_origin
                        + Vec3::new(
                            f32::from(u8::try_from(col).unwrap()),
                            f32::from(u8::try_from(row_num).unwrap()),
                            0.,
                        ),
                    NOMINO_COLLIDER_GROUP.into(),
                    None,
                    |color_id| {
                        let color = color_wrapper.get(color_id).unwrap();
                        color_block_count_map[*color] += 1;
                        *cell = true;
                        block_count += 1;

                        false
                    },
                );
            }
        }
        color_block_count_map.sort_unstable_by(|a, b| b.cmp(a));

        let resized_bag_matrix = bag_matrix
            .iter()
            .map(|row| &row[..width])
            .take(height)
            .collect::<SmallVec<[_; LARGEST_BAG_HEIGHT]>>();
        let total_bag_score = score_bag(
            &resized_bag_matrix,
            block_count,
            &color_block_count_map,
            bag_size.capacity(),
        );
        let bag_score = current_score.score_map.entry(*bag).or_insert(0);
        let diff = (i32::from(total_bag_score) - i32::from(*bag_score)) as isize;

        *bag_score = total_bag_score;
        current_score.points = (isize::try_from(current_score.points).unwrap() + diff)
            .try_into()
            .unwrap();
        current_score.all_time_points = (isize::try_from(current_score.all_time_points).unwrap()
            + diff)
            .try_into()
            .unwrap();
    }
}

fn reset_score(
    mut level_started: EventReader<LevelStarted>,
    mut current_score: ResMut<CurrentScore>,
    mut prev_level: Local<u16>,
) {
    if let Some(started) = level_started.iter().last() {
        if **started == *prev_level {
            current_score.all_time_points -= current_score.points;
        }

        current_score.points = 0;
        current_score.score_map.clear();

        *prev_level = **started;
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn score_bag(
    bag_matrix: &[impl AsRef<[bool]>],
    block_count: u8,
    color_block_count_map: &[u8],
    capacity: u8,
) -> u16 {
    debug_assert!(color_block_count_map.is_sorted_by(|a, b| Some(b.cmp(a))));
    debug_assert_eq!(color_block_count_map.iter().sum::<u8>(), block_count);
    debug_assert_eq!(
        bag_matrix
            .iter()
            .flat_map(AsRef::as_ref)
            .map(|b| u8::from(*b))
            .sum::<u8>(),
        block_count
    );
    debug_assert_eq!(
        capacity as usize,
        bag_matrix.len() * bag_matrix[0].as_ref().len()
    );

    let num_holes = count_holes(bag_matrix, block_count, capacity);
    let base_score = calculate_base_score(color_block_count_map, capacity);
    let multiplier = calculate_bag_fill_multiplier(block_count, capacity);
    let hole_penalty = u16::from(num_holes) * BLOCK_POINT_VALUE;

    (f32::from(multiplier) * (base_score - f32::from(hole_penalty))).round() as u16
}

fn count_holes(matrix: &[impl AsRef<[bool]>], block_count: u8, capacity: u8) -> u8 {
    capacity - block_count - get_connected_empties_count(matrix)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct RowCol(u8, u8);

impl RowCol {
    const fn left(self) -> Option<Self> {
        if self.1 > 0 {
            Some(Self(self.0, self.1 - 1))
        } else {
            None
        }
    }

    const fn right(self, max: u8) -> Option<Self> {
        if self.1 < max {
            Some(Self(self.0, self.1 + 1))
        } else {
            None
        }
    }

    const fn up(self, max: u8) -> Option<Self> {
        if self.0 < max {
            Some(Self(self.0 + 1, self.1))
        } else {
            None
        }
    }

    const fn down(self) -> Option<Self> {
        if self.0 > 0 {
            Some(Self(self.0 - 1, self.1))
        } else {
            None
        }
    }
}

/// Generates a vector containing the coordinates of all the empty spaces in the
/// bag that are connected to an empty space on the top row.
fn get_connected_empties_count(matrix: &[impl AsRef<[bool]>]) -> u8 {
    let mut connected_to_top = 0;
    let mut touched = HashSet::<RowCol>::with_capacity(LARGEST_BAG_CAPACITY);
    let mut frontier = VecDeque::<RowCol>::with_capacity(LARGEST_BAG_CAPACITY);
    let top_row = u8::try_from(matrix.len() - 1).unwrap();

    for (i, filled) in matrix.last().unwrap().as_ref().iter().enumerate() {
        let i = u8::try_from(i).unwrap();
        if !filled {
            let block = RowCol(top_row, i);
            connected_to_top += 1;

            if let Some(neighbor) = block.down() {
                frontier.push_back(neighbor);
                touched.insert(neighbor);
            }
        }

        touched.insert(RowCol(top_row, i));
    }

    while let Some(block) = frontier.pop_front() {
        let row = matrix[block.0 as usize].as_ref();
        let filled = row[block.1 as usize];

        if filled {
            continue;
        }
        connected_to_top += 1;

        let mut touch_neighbor = |neighbor| {
            if touched.insert(neighbor) {
                frontier.push_back(neighbor);
            }
        };

        block.left().map(&mut touch_neighbor);
        block
            .right((row.len() - 1).try_into().unwrap())
            .map(&mut touch_neighbor);
        block.up(top_row).map(&mut touch_neighbor);
        block.down().map(&mut touch_neighbor);
    }

    connected_to_top
}

fn calculate_base_score(color_map: &[u8], capacity: u8) -> f32 {
    let mut score = 0.;

    for (i, color_count) in color_map.iter().enumerate() {
        if *color_count == 0 {
            break;
        }

        let perfect_bag_bonus = *color_count == capacity;

        let mut raw_points = u16::from(*color_count) * BLOCK_POINT_VALUE;
        if perfect_bag_bonus {
            raw_points += 100;
        }
        score += f32::from(raw_points) * (1. + 1. / f32::from(1 + u8::try_from(i).unwrap()));
    }

    score
}

fn calculate_bag_fill_multiplier(block_count: u8, capacity: u8) -> u16 {
    debug_assert_eq!(capacity % 4, 0);
    debug_assert!(block_count <= capacity);

    let threshold_1 = capacity / 2 + 2;
    let threshold_2 = (capacity / 4) * 3 + 2;
    if (0..threshold_1).contains(&block_count) {
        1
    } else if (threshold_1..threshold_2).contains(&block_count) {
        2
    } else if (threshold_2..capacity).contains(&block_count) {
        5
    } else {
        10
    }
}

fn reclaim_memory(
    mut current_score: ResMut<CurrentScore>,
    mut filled_bags: EventReader<BagFilled>,
) {
    for bag in filled_bags.iter() {
        current_score.score_map.remove(&**bag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bag_gets_zero() {
        let (bag, block_count) = to_matrix(
            "
            000 000
            000 000
            000 000
            000 000
            000 000
            000 000
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = block_count;

        assert_eq!(0, score_bag(&bag, block_count, &color_map, 36));
    }

    #[test]
    fn full_bag_gets_max_score() {
        let (bag, block_count) = to_matrix(
            "
            111 111
            111 111
            111 111
            111 111
            111 111
            111 111
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = block_count;

        assert_eq!(20000, score_bag(&bag, block_count, &color_map, 36));
    }

    #[test]
    fn bag_gets_score_without_multiplier() {
        let (bag, block_count) = to_matrix(
            "
            000 000
            000 000
            000 001
            111 111
            111 111
            111 111
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = block_count;

        assert_eq!(950, score_bag(&bag, block_count, &color_map, 36));
    }

    #[test]
    fn bag_gets_2x_score() {
        let (bag, block_count) = to_matrix(
            "
            000 000
            000 111
            111 111
            111 111
            111 111
            111 111
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = block_count;

        assert_eq!(2700, score_bag(&bag, block_count, &color_map, 36));
    }

    #[test]
    fn bag_gets_5x_score() {
        let (bag, block_count) = to_matrix(
            "
            110 111
            111 111
            111 111
            111 111
            111 111
            111 111
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = block_count;

        assert_eq!(8750, score_bag(&bag, block_count, &color_map, 36));
    }

    #[test]
    fn bag_with_deep_open_space_isnt_detected_as_hole() {
        let (bag, block_count) = to_matrix(
            "
            110 111
            100 001
            111 011
            101 001
            101 011
            100 001
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = block_count;

        assert_eq!(2100, score_bag(&bag, block_count, &color_map, 36));
    }

    #[test]
    fn bag_with_holes_suffers_penalty() {
        let (bag, block_count) = to_matrix(
            "
            111 111
            100 001
            111 011
            101 001
            101 011
            100 001
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = block_count;

        assert_eq!(1500, score_bag(&bag, block_count, &color_map, 36));
    }

    #[test]
    fn bag_with_multiple_colors_reduces_score() {
        let (bag, block_count) = to_matrix(
            "
            111 111
            100 001
            111 011
            101 001
            101 011
            100 001
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = 15;
        color_map[1] = 7;

        assert_eq!(1325, score_bag(&bag, block_count, &color_map, 36));
    }

    #[test]
    fn small_full_bag_returns_max_score() {
        let (bag, block_count) = to_matrix(
            "
            000 000
            000 000
            000 000
            000 000
            111 100
            111 100
        ",
        );
        let mut color_map = [0; NominoColor::COUNT];
        color_map[0] = block_count;

        assert_eq!(
            6000,
            score_bag(
                &bag.iter().map(|row| &row[..4]).take(2).collect::<Vec<_>>(),
                block_count,
                &color_map,
                8,
            )
        );
    }

    fn to_matrix(bag: &str) -> ([[bool; LARGEST_BAG_WIDTH]; LARGEST_BAG_HEIGHT], u8) {
        let bag: String = bag.chars().filter(|c| !c.is_whitespace()).rev().collect();
        let mut matrix = [[false; LARGEST_BAG_WIDTH]; LARGEST_BAG_HEIGHT];

        let mut count = 0;
        for (i, c) in bag.chars().enumerate() {
            if c == '1' {
                let row = &mut matrix[i / matrix.len()];
                row[row.len() - (i % row.len()) - 1] = true;
                count += 1;
            }
        }

        (matrix, count)
    }
}
