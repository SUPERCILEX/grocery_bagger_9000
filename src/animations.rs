use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_tweening::{lens::TransformRotationLens, *};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, cleanup_animations::<Transform>);
    }
}

#[derive(Component, Deref)]
pub struct Original<T: Component>(T);

#[derive(Bundle)]
pub struct AnimationBundle<T: Component> {
    animator: Animator<T>,
    original: Original<T>,
}

pub fn error_shake(current: Transform) -> AnimationBundle<Transform> {
    let wiggle = Quat::from_rotation_z(PI / 16.);
    AnimationBundle {
        animator: Animator::new(Sequence::new([
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(25),
                TransformRotationLens {
                    start: current.rotation,
                    end: current.rotation * wiggle.inverse(),
                },
            ),
            Tween::new(
                EaseMethod::Linear,
                TweeningType::PingPongTimes(3),
                Duration::from_millis(50),
                TransformRotationLens {
                    start: current.rotation * wiggle.inverse(),
                    end: current.rotation * wiggle,
                },
            ),
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(25),
                TransformRotationLens {
                    start: current.rotation * wiggle,
                    end: current.rotation,
                },
            )
            .with_completed_event(true, 0),
        ])),
        original: Original(current),
    }
}

fn cleanup_animations<T: Component>(
    mut commands: Commands,
    mut completed_animations: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, .. } in completed_animations.iter() {
        commands
            .entity(*entity)
            .remove_bundle::<AnimationBundle<T>>();
    }
}
