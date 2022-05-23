use std::collections::{HashMap, HashSet, VecDeque};

use bevy::{math::const_vec3, prelude::*, transform::TransformSystem::TransformPropagate};
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    bags::{BagMarker, BAG_CAPACITY, BAG_ORIGIN},
    colors::NominoColor,
    levels::{LevelInitLabel, LevelStarted},
    nominos::{NominoMarker, PiecePlaced, NOMINO_COLLIDER_GROUP},
};

pub struct ScoringPlugin;

const BLOCK_POINT_VALUE: u16 = 25;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentScore>();

        app.add_system_to_stage(CoreStage::PostUpdate, score_bags.after(TransformPropagate));
        app.add_system_to_stage(CoreStage::PreUpdate, reset_score.after(LevelInitLabel));
    }
}

#[derive(Debug, Default)]
pub struct CurrentScore {
    pub points: u16,
    score_map: HashMap<Entity, u16>,
}

fn score_bags(
    mut piece_placements: EventReader<PiecePlaced>,
    bags: Query<&GlobalTransform, With<BagMarker>>,
    color_wrapper: Query<&NominoColor, With<NominoMarker>>,
    rapier_context: Res<RapierContext>,
    mut current_score: ResMut<CurrentScore>,
) {
    for PiecePlaced { bag, .. } in piece_placements.iter() {
        let bag_coords = *bags.get(*bag).unwrap();

        let block_origin = bag_coords.translation - BAG_ORIGIN + const_vec3!([0.5, 0.5, 0.]);

        let mut bag_matrix = [[false; 6]; 6];
        let mut block_count = 0u8;
        let mut color_block_count_map = [0u8; NominoColor::COUNT];
        for (row_num, row) in bag_matrix.iter_mut().enumerate() {
            for (col, cell) in row.iter_mut().enumerate() {
                rapier_context.intersections_with_point(
                    block_origin + Vec3::new(col as f32, row_num as f32, 0.),
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

        let total_bag_score = score_bag(&bag_matrix, block_count, &color_block_count_map);
        let bag_score = current_score.score_map.entry(*bag).or_insert(0);
        let diff = total_bag_score as i16 - *bag_score as i16;

        *bag_score = total_bag_score;
        current_score.points = (current_score.points as i16 + diff) as u16;
    }
}

fn reset_score(
    mut level_loaded: EventReader<LevelStarted>,
    mut current_score: ResMut<CurrentScore>,
) {
    if level_loaded.iter().count() > 0 {
        current_score.points = 0;
        current_score.score_map.clear();
    }
}

fn score_bag(
    bag_matrix: &[[bool; 6]; 6],
    block_count: u8,
    color_block_count_map: &[u8; NominoColor::COUNT],
) -> u16 {
    debug_assert!(color_block_count_map.is_sorted_by(|a, b| Some(b.cmp(a))));
    debug_assert_eq!(color_block_count_map.iter().sum::<u8>(), block_count);
    debug_assert_eq!(
        bag_matrix.iter().flatten().map(|b| *b as u8).sum::<u8>(),
        block_count
    );

    let num_holes = count_holes(bag_matrix, block_count);
    let base_score = calculate_base_score(color_block_count_map);
    let multiplier = calculate_bag_fill_multiplier(block_count);
    let hole_penalty = num_holes as u16 * BLOCK_POINT_VALUE;

    (multiplier as f32 * (base_score - hole_penalty as f32)).round() as u16
}

fn count_holes(matrix: &[[bool; 6]; 6], block_count: u8) -> u8 {
    BAG_CAPACITY as u8 - block_count - get_connected_empties(matrix).len() as u8
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct RowCol(usize, usize);

impl RowCol {
    fn left(&self) -> Option<Self> {
        if self.0 > 0 {
            Some(RowCol(self.0 - 1, self.1))
        } else {
            None
        }
    }

    fn right(&self, max: usize) -> Option<Self> {
        if self.0 < max {
            Some(RowCol(self.0 + 1, self.1))
        } else {
            None
        }
    }

    fn up(&self) -> Option<Self> {
        if self.1 > 0 {
            Some(RowCol(self.0, self.1 - 1))
        } else {
            None
        }
    }

    fn down(&self, max: usize) -> Option<Self> {
        if self.1 < max {
            Some(RowCol(self.0, self.1 + 1))
        } else {
            None
        }
    }
}

/// Generates a vector containing the coordinates of all the empty spaces in the
/// bag that are connected to an empty space on the top row.
fn get_connected_empties(matrix: &[[bool; 6]; 6]) -> SmallVec<[RowCol; BAG_CAPACITY]> {
    let mut connected_to_top = SmallVec::<[RowCol; BAG_CAPACITY]>::new();
    let mut touched = HashSet::<RowCol>::with_capacity(BAG_CAPACITY);
    let mut frontier = VecDeque::<RowCol>::with_capacity(BAG_CAPACITY);
    let top_row = matrix.len() - 1;

    for (i, filled) in matrix.last().unwrap().iter().enumerate() {
        if !filled {
            connected_to_top.push(RowCol(top_row, i));
            frontier.push_back(RowCol(top_row - 1, i));
            touched.insert(RowCol(top_row - 1, i));
        }

        touched.insert(RowCol(top_row, i));
    }

    while let Some(block) = frontier.pop_front() {
        let row = block.0;
        let col = block.1;
        let filled = matrix[row][col];

        if filled {
            continue;
        }
        connected_to_top.push(block);

        let mut touch_neighbor = |neighbor| {
            if touched.insert(neighbor) {
                frontier.push_back(neighbor);
            }
        };
        block.left().map(&mut touch_neighbor);
        block.right(top_row).map(&mut touch_neighbor);
        block.up().map(&mut touch_neighbor);
        block.down(top_row).map(&mut touch_neighbor);
    }

    connected_to_top
}

fn calculate_base_score(color_map: &[u8; NominoColor::COUNT]) -> f32 {
    let mut score = 0.;

    for (i, color_count) in color_map.iter().enumerate() {
        let color_count = *color_count as u16;
        if color_count == 0 {
            break;
        }

        let perfect_bag_bonus = color_count == BAG_CAPACITY as u16;

        let mut raw_points = color_count * BLOCK_POINT_VALUE;
        if perfect_bag_bonus {
            raw_points += 100;
        }
        score += raw_points as f32 * (1. + 1. / (1 + i) as f32);
    }

    score
}

const fn calculate_bag_fill_multiplier(block_count: u8) -> u16 {
    match block_count {
        0..=19 => 1,
        20..=27 => 2,
        28..=35 => 5,
        36 => 10,
        _ => panic!("More blocks than can fit in a bag"),
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

        assert_eq!(0, score_bag(&bag, block_count, &color_map));
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

        assert_eq!(20000, score_bag(&bag, block_count, &color_map));
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

        assert_eq!(950, score_bag(&bag, block_count, &color_map));
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

        assert_eq!(2700, score_bag(&bag, block_count, &color_map));
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

        assert_eq!(8750, score_bag(&bag, block_count, &color_map));
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

        assert_eq!(2100, score_bag(&bag, block_count, &color_map));
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

        assert_eq!(1500, score_bag(&bag, block_count, &color_map));
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

        assert_eq!(1325, score_bag(&bag, block_count, &color_map));
    }

    fn to_matrix(bag: &str) -> ([[bool; 6]; 6], u8) {
        let bag: String = bag.chars().filter(|c| !c.is_whitespace()).rev().collect();
        let mut matrix = [[false; 6]; 6];

        let mut count = 0;
        for (i, c) in bag.chars().enumerate() {
            if c == '1' {
                matrix[i / matrix.len()][i % matrix.len()] = true;
                count += 1;
            }
        }

        (matrix, count)
    }
}
