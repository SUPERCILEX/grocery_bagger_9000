use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    bags, bags::BAG_OFFSET, colors::NominoColor, nominos::NOMINO_COLLIDER_GROUP,
    piece_movement::PiecePlaced,
};

pub struct ScoringPlugin;

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
        let mut bag_coords = *bags.get(*bag).unwrap();

        let block_origin =
            bag_coords.translation + Vec3::new(-bags::BAG_OFFSET, -bags::BAG_OFFSET, 0.);

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
        dbg!(color_block_count_map);
        dbg!(bag_matrix);
        dbg!(block_count);
    }
}

fn score_blocks(something: u32) -> u32 {
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
