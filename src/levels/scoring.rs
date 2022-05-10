use std::collections::{HashSet, VecDeque};

use bevy::{math::const_vec3, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{
    bags,
    bags::{BAG_CAPACITY, BAG_ORIGIN},
    colors::NominoColor,
    nominos::{PiecePlaced, NOMINO_COLLIDER_GROUP},
};

pub struct ScoringPlugin;

const BLOCK_POINT_VALUE: u32 = 25;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, score_bags);
    }
}

fn score_bags(
    mut piece_placements: EventReader<PiecePlaced>,
    bags: Query<&Transform>,
    color_wrapper: Query<&NominoColor>,
    rapier_context: Res<RapierContext>,
) {
    for PiecePlaced { bag, .. } in piece_placements.iter() {
        let bag_coords = *bags.get(*bag).unwrap();

        let block_origin = bag_coords.translation - BAG_ORIGIN + const_vec3!([0.5, 0.5, 0.]);

        let mut color_block_count_map = [0u8; NominoColor::COUNT];
        let mut bag_matrix = [[false; 6]; 6];
        let mut block_count: u32 = 0;
        // iterate rows
        for j in 0..6 {
            //iterate columns
            for i in 0..6 {
                rapier_context.intersections_with_point(
                    block_origin + Vec3::new(i as f32, j as f32, 0.),
                    NOMINO_COLLIDER_GROUP.into(),
                    None,
                    |color_id| {
                        let color = color_wrapper.get(color_id).unwrap();
                        color_block_count_map[color.id()] += 1;
                        bag_matrix[j][i] = true;
                        block_count += 1;
                        true
                    },
                );
            }
        }
        // Entire bag has been counted. Time to do stuff with the totals.
        // dbg!(color_block_count_map);
        // dbg!(bag_matrix);
        // dbg!(block_count);
        let num_holes = count_holes(&bag_matrix, block_count);
        let hole_penalty = num_holes * BLOCK_POINT_VALUE;
        // dbg!(num_holes);
        let color_score = calculate_color_score(color_block_count_map, block_count);
        dbg!(color_score);
        // dbg!(color_score);
        let mult = calculate_bag_fill_multiplier(block_count);
        dbg!(mult);
        let total_bag_score = mult * (color_score - hole_penalty);
        dbg!(total_bag_score);
    }
}

fn count_holes(matrix: &[[bool; 6]; 6], block_count: u32) -> u32 {
    return (BAG_CAPACITY as u32) - block_count - (get_connected_empties(matrix).len() as u32);
}

// generates a vector containing the coordinates of all the empty spaces in the
// bag that are connected to an empty space on the top row.
fn get_connected_empties(matrix: &[[bool; 6]; 6]) -> Vec<(usize, usize)> {
    let mut connected_to_top: Vec<(usize, usize)> = Vec::new();
    let mut touched: HashSet<(usize, usize)> = HashSet::with_capacity(BAG_CAPACITY);
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::with_capacity(BAG_CAPACITY);
    for i in 0..6 {
        if !matrix[5][i] {
            // empty space
            connected_to_top.push((5, i));
            frontier.push_back((4, i));
        }
        touched.insert((5, i));
        touched.insert((4, i));
    }
    while let Some(block) = frontier.pop_front() {
        if !matrix[block.0][block.1] {
            // empty space in bag, and connected to the top
            // store connected node
            connected_to_top.push(block);
            // add any neighbors that are within the legal range
            // to our frontier
            if block.0 > 0 {
                let below = (block.0 - 1, block.1);
                touch_neighbor(below, &mut frontier, &mut touched);
            }
            if block.0 < 4 {
                // already explored matrix[5][x]
                let above = (block.0 + 1, block.1);
                touch_neighbor(above, &mut frontier, &mut touched);
            }
            if block.1 > 0 {
                let left = (block.0, block.1 - 1);
                touch_neighbor(left, &mut frontier, &mut touched);
            }
            if block.1 < 5 {
                let right = (block.0, block.1 + 1);
                touch_neighbor(right, &mut frontier, &mut touched);
            }
        }
    }
    return connected_to_top;
}

fn touch_neighbor(
    neighbor: (usize, usize),
    frontier: &mut VecDeque<(usize, usize)>,
    touched: &mut HashSet<(usize, usize)>,
) {
    if !touched.contains(&neighbor) {
        frontier.push_back(neighbor);
        touched.insert(neighbor);
    }
}

fn calculate_color_score(color_map: [u8; NominoColor::COUNT], block_count: u32) -> u32 {
    let mut ranked_colors = Vec::from(color_map);
    ranked_colors.sort();
    ranked_colors.reverse();
    let mut color_score: u32 = 0;
    let mut total_blocks: usize = 0;
    for x in 0..ranked_colors.len() {
        let current_color_count: u32 = ranked_colors[x].into();
        if current_color_count == 0 {
            break;
        }
        total_blocks += current_color_count as usize;
        let perfect_bag_bonus = x == 0 && current_color_count == 36;
        let mut raw_points = current_color_count * BLOCK_POINT_VALUE;
        if perfect_bag_bonus {
            raw_points += 100;
        }
        let adjusted_points = raw_points * (1 + 1 / (1 + (x as u32)));
        dbg!(adjusted_points);
        color_score += adjusted_points;
    }
    if total_blocks == BAG_CAPACITY {
        color_score += 100;
    }
    return color_score;
}

fn calculate_bag_fill_multiplier(block_count: u32) -> u32 {
    match block_count as usize {
        0..=19 => return 1,
        20..=32 => return 2,
        33..=BAG_CAPACITY => return 5,
        _ => println!("You shouldn't be here!"), //should be impossible
    }
    return 0;
}

fn score_blocks(num: u32) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blah_blah() {
        assert_eq!(1, score_blocks(0));
    }
}