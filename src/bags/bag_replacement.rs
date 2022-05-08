use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    bags,
    bags::BAG_LID_COLLIDER_GROUP,
    conveyor_belt,
    nominos::{PiecePlaced, NOMINO_COLLIDER_GROUP},
};

pub struct BagReplacementPlugin;

impl Plugin for BagReplacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, replace_full_bags);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct BagPieces(pub SmallVec<[Entity; conveyor_belt::MAX_NUM_PIECES]>);

fn replace_full_bags(
    mut commands: Commands,
    mut piece_placements: EventReader<PiecePlaced>,
    bags: Query<&Transform>,
    mut bag_pieces: Query<&mut BagPieces>,
    rapier_context: Res<RapierContext>,
    piece_colliders: Query<(&Transform, &Collider)>,
    lid_collider_bag: Query<&Parent>,
) {
    for PiecePlaced { piece, bag } in piece_placements.iter() {
        let bag_pieces = &mut *bag_pieces.get_mut(*bag).unwrap();
        bag_pieces.push(*piece);

        {
            let (transform, collider) = piece_colliders.get(*piece).unwrap();
            let bag_overflowing = rapier_context
                .intersection_with_shape(
                    transform.translation,
                    transform.rotation,
                    collider,
                    BAG_LID_COLLIDER_GROUP.into(),
                    Some(&|entity| **lid_collider_bag.get(entity).unwrap() == *bag),
                )
                .is_some();
            if bag_overflowing {
                replace_bag(&mut commands, bag_pieces);
                continue;
            }
        }

        let mut bag_coords = *bags.get(*bag).unwrap();
        bag_coords.translation += Vec3::new(0.5 - bags::RADIUS, bags::RADIUS - 0.5, 0.);

        let mut top_row_full = true;
        for i in 0..6 {
            let mut intersection = false;
            rapier_context.intersections_with_point(
                bag_coords.translation + Vec3::new(i as f32, 0., 0.),
                NOMINO_COLLIDER_GROUP.into(),
                None,
                |_| {
                    intersection = true;
                    false
                },
            );

            if !intersection {
                top_row_full = false;
                break;
            }
        }
        if top_row_full {
            replace_bag(&mut commands, bag_pieces);
        }
    }
}

fn replace_bag(commands: &mut Commands, bag_pieces: &mut BagPieces) {
    for piece in bag_pieces.iter() {
        commands.entity(*piece).despawn_recursive();
    }
    bag_pieces.clear();
}
