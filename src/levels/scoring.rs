use std::collections::{HashMap, HashSet, VecDeque};

use bevy::prelude::*;

use crate::{
    bags::{
        BagChangeDetectionSystems, BagChanged, BagFilled, BagMarker, BagReplacementSystems,
        BagSize, BAG_SIZE_LARGE,
    },
    levels::{LevelSpawnStage, LevelStarted},
    nominos::NominoColor,
};

pub struct ScoringPlugin;

const BLOCK_POINT_VALUE: u16 = 25;
const LARGE_BAG_CAPACITY: usize = BAG_SIZE_LARGE.capacity() as usize;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentScore>();

        app.add_event::<ScoreChanged>();

        app.add_system(
            score_bags
                .label(ScoringSystems)
                .after(BagChangeDetectionSystems)
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

#[derive(Debug)]
pub struct ScoreChanged {
    pub cause: Entity,
    pub diff: isize,
}

fn score_bags(
    mut bag_changes: EventReader<BagChanged>,
    bags: Query<&BagSize, With<BagMarker>>,
    mut current_score: ResMut<CurrentScore>,
    mut score_changes: EventWriter<ScoreChanged>,
) {
    for BagChanged { bag, blocks } in bag_changes.iter() {
        let capacity = bags.get(*bag).unwrap().capacity();

        let total_bag_score = score_bag(blocks, capacity);
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

        score_changes.send(ScoreChanged { cause: *bag, diff });
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
fn score_bag(bag_matrix: &[impl AsRef<[Option<NominoColor>]>], capacity: u8) -> u16 {
    debug_assert_eq!(
        capacity as usize,
        bag_matrix.len() * bag_matrix[0].as_ref().len()
    );

    let block_count = bag_matrix
        .iter()
        .flat_map(AsRef::as_ref)
        .map(|b| b.map_or(0, |_| 1))
        .sum::<u8>();

    let mut color_block_count_map = [0u8; NominoColor::COUNT];
    for row in bag_matrix {
        for color in row.as_ref().iter().flatten() {
            color_block_count_map[*color] += 1;
        }
    }
    color_block_count_map.sort_unstable_by(|a, b| b.cmp(a));

    let num_holes = count_holes(bag_matrix, block_count, capacity);
    let base_score = calculate_base_score(&color_block_count_map, capacity);
    let multiplier = calculate_bag_fill_multiplier(block_count, capacity);
    let hole_penalty = u16::from(num_holes) * BLOCK_POINT_VALUE;

    (f32::from(multiplier) * (base_score - f32::from(hole_penalty))).round() as u16
}

fn count_holes(matrix: &[impl AsRef<[Option<NominoColor>]>], block_count: u8, capacity: u8) -> u8 {
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
fn get_connected_empties_count(matrix: &[impl AsRef<[Option<NominoColor>]>]) -> u8 {
    let mut connected_to_top = 0;
    let mut touched = HashSet::<RowCol>::with_capacity(LARGE_BAG_CAPACITY);
    let mut frontier = VecDeque::<RowCol>::with_capacity(LARGE_BAG_CAPACITY);
    let top_row = u8::try_from(matrix.len() - 1).unwrap();

    for (i, filled) in matrix.last().unwrap().as_ref().iter().enumerate() {
        let i = u8::try_from(i).unwrap();
        if filled.is_none() {
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
        let filled = row[block.1 as usize].is_some();

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
    use num_traits::FromPrimitive;

    use super::*;

    #[test]
    fn empty_bag_gets_zero() {
        let bag = to_matrix(
            6,
            "
            000 000
            000 000
            000 000
            000 000
            000 000
            000 000
        ",
        );

        assert_eq!(0, score_bag(&bag, 36));
    }

    #[test]
    fn full_bag_gets_max_score() {
        let bag = to_matrix(
            6,
            "
            111 111
            111 111
            111 111
            111 111
            111 111
            111 111
        ",
        );

        assert_eq!(20000, score_bag(&bag, 36));
    }

    #[test]
    fn bag_gets_score_without_multiplier() {
        let bag = to_matrix(
            6,
            "
            000 000
            000 000
            000 001
            111 111
            111 111
            111 111
        ",
        );

        assert_eq!(950, score_bag(&bag, 36));
    }

    #[test]
    fn bag_gets_2x_score() {
        let bag = to_matrix(
            6,
            "
            000 000
            000 111
            111 111
            111 111
            111 111
            111 111
        ",
        );

        assert_eq!(2700, score_bag(&bag, 36));
    }

    #[test]
    fn bag_gets_5x_score() {
        let bag = to_matrix(
            6,
            "
            110 111
            111 111
            111 111
            111 111
            111 111
            111 111
        ",
        );

        assert_eq!(8750, score_bag(&bag, 36));
    }

    #[test]
    fn bag_with_deep_open_space_isnt_detected_as_hole() {
        let bag = to_matrix(
            6,
            "
            110 111
            100 001
            111 011
            101 001
            101 011
            100 001
        ",
        );

        assert_eq!(2100, score_bag(&bag, 36));
    }

    #[test]
    fn bag_with_holes_suffers_penalty() {
        let bag = to_matrix(
            6,
            "
            111 111
            100 001
            111 011
            101 001
            101 011
            100 001
        ",
        );

        assert_eq!(1500, score_bag(&bag, 36));
    }

    #[test]
    fn bag_with_multiple_colors_reduces_score() {
        let bag = to_matrix(
            6,
            "
            111 111
            100 002
            222 022
            202 002
            202 022
            200 002
        ",
        );

        assert_eq!(1325, score_bag(&bag, 36));
    }

    #[test]
    fn small_full_bag_returns_max_score() {
        let bag = to_matrix(
            4,
            "
            1111
            1111
        ",
        );

        assert_eq!(6000, score_bag(&bag, 8));
    }

    fn to_matrix(cols: usize, bag: &str) -> Vec<Vec<Option<NominoColor>>> {
        let bag: String = bag.chars().filter(|c| !c.is_whitespace()).rev().collect();
        let mut matrix = Vec::new();

        for (i, c) in bag.chars().enumerate() {
            if i % cols == 0 {
                matrix.push(Vec::new());
            };
            let row = &mut matrix[i / cols];

            row.push(if c == '0' {
                None
            } else {
                let id = c.to_digit(10).unwrap() - 1;
                Some(NominoColor::from_u32(id).unwrap())
            });
        }

        matrix
    }
}
