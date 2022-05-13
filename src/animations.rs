use std::{f32::consts::PI, time::Duration};

use bevy::{math::const_vec3, prelude::*};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens, TransformScaleLens},
    *,
};
use bitflags::bitflags;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSpeed>();

        app.add_system(
            change_animation_speed::<Transform>.before(AnimationSystem::AnimationUpdate),
        );
        app.add_system_to_stage(CoreStage::PostUpdate, cleanup_animations::<Transform>);
    }
}

#[derive(Deref, DerefMut)]
pub struct GameSpeed(f32);

impl Default for GameSpeed {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Component, Deref)]
pub struct Original<T: Component>(T);

#[derive(Bundle)]
pub struct UndoableAnimationBundle<T: Component> {
    animator: Animator<T>,
    original: Original<T>,
}

bitflags! {
    pub struct AnimationEvent: u64 {
        const COMPLETED = 1;
        const BAG_OFF_SCREEN = 1 << 1;
    }
}

type DynTweenable = Box<dyn Tweenable<Transform> + Send + Sync + 'static>;

struct NoopLens;

impl<T> Lens<T> for NoopLens {
    fn lerp(&mut self, _: &mut T, _: f32) {}
}

pub fn error_shake(current: Transform, speed: &GameSpeed) -> UndoableAnimationBundle<Transform> {
    let wiggle = Quat::from_rotation_z(PI / 16.);

    UndoableAnimationBundle {
        animator: Animator::new(Sequence::new([
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(25),
                TransformRotationLens {
                    start: current.rotation,
                    end: current.rotation * wiggle.inverse(),
                },
            )
            .with_speed(**speed),
            Tween::new(
                EaseMethod::Linear,
                TweeningType::PingPongTimes(3),
                Duration::from_millis(50),
                TransformRotationLens {
                    start: current.rotation * wiggle.inverse(),
                    end: current.rotation * wiggle,
                },
            )
            .with_speed(**speed),
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(25),
                TransformRotationLens {
                    start: current.rotation * wiggle,
                    end: current.rotation,
                },
            )
            .with_speed(**speed)
            .with_completed_event(true, AnimationEvent::COMPLETED.bits()),
        ])),
        original: Original(current),
    }
}

pub fn bag_enter(from: Transform, to: Transform, speed: &GameSpeed) -> Animator<Transform> {
    Animator::new(Sequence::new([
        Box::new(
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(500),
                NoopLens,
            )
            .with_speed(**speed),
        ) as DynTweenable,
        Box::new(Tracks::new([
            Box::new(
                Tween::new(
                    EaseMethod::CustomFunction(|x: f32| {
                        const C1: f32 = 1.70158;
                        const C3: f32 = C1 + 1.;

                        let x1 = x - 1.;
                        let x2 = x1 * x1;
                        1. + C3 * x1 * x2 + C1 * x2
                    }),
                    TweeningType::Once,
                    Duration::from_millis(200),
                    TransformScaleLens {
                        start: from.scale,
                        end: to.scale,
                    },
                )
                .with_speed(**speed)
                .with_completed_event(true, AnimationEvent::COMPLETED.bits()),
            ) as DynTweenable,
            Box::new(Sequence::new([
                Tween::new(
                    EaseMethod::EaseFunction(EaseFunction::CircularIn),
                    TweeningType::Once,
                    Duration::from_millis(100),
                    TransformPositionLens {
                        start: from.translation,
                        end: from.translation + const_vec3!([0., 2., 0.]),
                    },
                )
                .with_speed(**speed),
                Tween::new(
                    EaseMethod::EaseFunction(EaseFunction::CircularOut),
                    TweeningType::Once,
                    Duration::from_millis(100),
                    TransformPositionLens {
                        start: from.translation + const_vec3!([0., 2., 0.]),
                        end: to.translation,
                    },
                )
                .with_speed(**speed),
            ])) as DynTweenable,
        ])) as DynTweenable,
    ]))
}

pub fn bag_exit(from: Transform, to: Transform, speed: &GameSpeed) -> Animator<Transform> {
    Animator::new(
        Tween::new(
            EaseMethod::CustomFunction(|x| {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.;

                let x2 = x * x;
                C3 * x * x2 - C1 * x2
            }),
            TweeningType::Once,
            Duration::from_millis(500),
            TransformPositionLens {
                start: from.translation,
                end: to.translation,
            },
        )
        .with_speed(**speed)
        .with_completed_event(
            true,
            (AnimationEvent::COMPLETED | AnimationEvent::BAG_OFF_SCREEN).bits(),
        ),
    )
}

fn change_animation_speed<T: Component>(
    game_speed: Res<GameSpeed>,
    mut animators: Query<&mut Animator<T>>,
) {
    if !game_speed.is_changed() {
        return;
    }

    for mut animator in animators.iter_mut() {
        animator.set_speed(**game_speed);
    }
}

fn cleanup_animations<T: Component>(
    mut commands: Commands,
    mut completed_animations: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, user_data } in completed_animations.iter() {
        if *user_data & AnimationEvent::COMPLETED.bits() != 0 {
            commands
                .entity(*entity)
                .remove_bundle::<UndoableAnimationBundle<T>>();
        }
    }
}
