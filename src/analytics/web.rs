use bevy::{math::const_vec3, prelude::*, tasks::AsyncComputeTaskPool};
use bevy_rapier3d::plugin::RapierContext;
use wasm_bindgen::prelude::*;

use crate::{
    bags::{BagMarker, BagSize},
    gb9000::GroceryBagger9000,
    levels::{CurrentScore, LevelFinished, LevelStarted, ScoringSystems},
    nominos::{NominoColor, NominoMarker, PiecePickedUp, PiecePlaced, NOMINO_COLLIDER_GROUP},
    robot::RobotOptions,
};

pub struct AnalyticsPlugin;

impl Plugin for AnalyticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);

        app.add_system(log_level_start);
        app.add_system(log_level_end);
        app.add_system(log_piece_picked_up.after(ScoringSystems));
        app.add_system(log_piece_placed.after(ScoringSystems));
    }
}

#[derive(Copy, Clone)]
enum VersionIds {
    WithRobot = 20,
    NoRobot = 21,
}

fn init(thread_pool: Res<AsyncComputeTaskPool>, robot_options: Res<RobotOptions>) {
    let robot_enabled = robot_options.enabled;
    thread_pool
        .spawn(async move {
            init_analytics(if robot_enabled {
                VersionIds::WithRobot
            } else {
                VersionIds::NoRobot
            } as u32);
        })
        .detach();
}

fn log_level_start(
    mut level_start: EventReader<LevelStarted>,
    thread_pool: Res<AsyncComputeTaskPool>,
    gb9000: Res<GroceryBagger9000>,
) {
    if level_start.iter().count() == 0 {
        return;
    }

    let level_no = gb9000.current_level as u32;
    thread_pool
        .spawn(async move {
            logLevelStart(level_no);
        })
        .detach();
}

fn log_level_end(
    mut level_end: EventReader<LevelFinished>,
    thread_pool: Res<AsyncComputeTaskPool>,
    gb9000: Res<GroceryBagger9000>,
) {
    if level_end.iter().count() == 0 {
        return;
    }

    let level_no = gb9000.current_level as u32;
    thread_pool
        .spawn(async move {
            logLevelEnd(level_no);
        })
        .detach();
}

fn log_piece_placed(
    mut piece_placed: EventReader<PiecePlaced>,
    current_score: Res<CurrentScore>,
    thread_pool: Res<AsyncComputeTaskPool>,
    rapier_context: Res<RapierContext>,
    bags: Query<(&GlobalTransform, &BagSize), With<BagMarker>>,
    colors: Query<&NominoColor, With<NominoMarker>>,
) {
    for PiecePlaced { bag, .. } in piece_placed.iter() {
        let (bag_coords, bag_size) = bags.get(*bag).unwrap();

        let width = bag_size.width();
        let height = bag_size.height();
        let block_origin = bag_coords.translation - bag_size.origin() + const_vec3!([0.5, 0.5, 0.]);

        let mut bag_representation = String::with_capacity(40);
        for row in 0..height {
            for col in 0..width {
                let mut color = None;
                rapier_context.intersections_with_point(
                    block_origin + Vec3::new(col as f32, row as f32, 0.),
                    NOMINO_COLLIDER_GROUP.into(),
                    None,
                    |piece_id| {
                        color = Some(colors.get(piece_id).unwrap());
                        false
                    },
                );

                if let Some(color) = color {
                    bag_representation.push_str(&format!("{}", *color as u32 + 1));
                } else {
                    bag_representation.push('0');
                }
            }
            bag_representation.push('\n');
        }

        let score = current_score.points;
        let bag_id = bag.to_bits();
        thread_pool
            .spawn(async move {
                logPiecePlaced(score, bag_representation, bag_id);
            })
            .detach();
    }
}

fn log_piece_picked_up(
    mut pieces_picked_up: EventReader<PiecePickedUp>,
    thread_pool: Res<AsyncComputeTaskPool>,
    current_score: Res<CurrentScore>,
    colors: Query<&NominoColor, With<NominoMarker>>,
) {
    for piece in pieces_picked_up.iter() {
        let score = current_score.points;
        let color = *colors.get(**piece).unwrap() as u32;
        thread_pool
            .spawn(async move {
                logPiecePickedUp(score, color);
            })
            .detach();
    }
}

#[wasm_bindgen]
extern "C" {
    fn init_analytics(version_no: u32);

    fn logLevelStart(level_id: u32);

    fn logLevelEnd(level_id: u32);

    fn logPiecePickedUp(score: usize, color: u32);

    fn logPiecePlaced(score: usize, bag_state: String, bag_id: u64);
}
