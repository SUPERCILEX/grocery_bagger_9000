use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{bags, colors::NominoColor, events::PiecePlaced, nominos::NOMINO_COLLIDER_GROUP};

pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, score_bags);
    }
}

fn score_bags(
    mut piece_placements: EventReader<PiecePlaced>,
    bags: Query<&Transform>,
    colors: Query<&NominoColor>,
    rapier_context: Res<RapierContext>,
) {
    for PiecePlaced { bag, .. } in piece_placements.iter() {
        let mut bag_coords = *bags.get(*bag).unwrap();
        bag_coords.translation += Vec3::new(0.5 - bags::RADIUS, bags::RADIUS - 0.5, 0.);

        let mut the_color_to_block_map_we_were_talking_about = [0u8; NominoColor::COUNT];
        // TODO iterate columns too
        for i in 0..6 {
            rapier_context.intersections_with_point(
                bag_coords.translation + Vec3::new(i as f32, 0., 0.),
                NOMINO_COLLIDER_GROUP.into(),
                None,
                |entity| {
                    let color = colors.get(entity).unwrap();
                    the_color_to_block_map_we_were_talking_about[color.id()] += 1;
                    dbg!(the_color_to_block_map_we_were_talking_about);
                    // TODO do something knowing there's an intersection
                    false
                },
            );
        }
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
