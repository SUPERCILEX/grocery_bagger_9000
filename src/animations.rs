use std::{f32::consts::PI, time::Duration};

use bevy::{math::const_vec3, prelude::*};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens, TransformScaleLens, UiPositionLens},
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
        app.add_system(change_animation_speed::<Style>.before(AnimationSystem::AnimationUpdate));

        app.add_system(cleanup_animations::<Transform>.after(AnimationSystem::AnimationUpdate));
        app.add_system(cleanup_animations::<Style>.after(AnimationSystem::AnimationUpdate));
        app.add_system(despawn_offscreen.after(AnimationSystem::AnimationUpdate));
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

#[derive(Component, Deref)]
pub struct Target<T: Component>(T);

#[derive(Bundle)]
pub struct UndoableAnimationBundle<T: Component> {
    animator: Animator<T>,
    original: Original<T>,
}

#[derive(Bundle)]
pub struct RedoableAnimationBundle<T: Component> {
    animator: Animator<T>,
    original: Target<T>,
}

bitflags! {
    pub struct AnimationEvent: u64 {
        const COMPLETED = 1;
        const OFFSCREEN = 1 << 1;
        const BAG = 1 << 2;
    }
}

// TODO remove and check for with_speed stuff after animation lib upgrade
type DynTweenable = Box<dyn Tweenable<Transform> + Send + Sync + 'static>;

struct TeleportLens<T>(T);

impl<T: Copy> Lens<T> for TeleportLens<T> {
    fn lerp(&mut self, target: &mut T, _: f32) {
        *target = self.0;
    }
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
        Box::new(Delay::new(Duration::from_millis(500))) as DynTweenable,
        Box::new(Tracks::new([
            Box::new(
                Tween::new(
                    EaseMethod::CustomFunction(ease_out_back),
                    TweeningType::Once,
                    Duration::from_millis(200),
                    TransformScaleLens {
                        start: from.scale,
                        end: to.scale,
                    },
                )
                .with_speed(**speed)
                .with_completed_event(
                    true,
                    (AnimationEvent::COMPLETED | AnimationEvent::BAG).bits(),
                ),
            ) as DynTweenable,
            Box::new(Sequence::new([
                Tween::new(
                    EaseFunction::CircularIn,
                    TweeningType::Once,
                    Duration::from_millis(100),
                    TransformPositionLens {
                        start: from.translation,
                        end: from.translation + const_vec3!([0., 2., 0.]),
                    },
                )
                .with_speed(**speed),
                Tween::new(
                    EaseFunction::CircularOut,
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
            EaseMethod::CustomFunction(ease_in_back),
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
            (AnimationEvent::COMPLETED | AnimationEvent::OFFSCREEN | AnimationEvent::BAG).bits(),
        ),
    )
}

pub fn piece_placed(current: Transform, speed: &GameSpeed) -> Animator<Transform> {
    let pop = current.scale + const_vec3!([0.2, 0.2, 0.2]);

    Animator::new(Sequence::new([
        Tween::new(
            EaseFunction::CircularIn,
            TweeningType::Once,
            Duration::from_millis(100),
            TransformScaleLens {
                start: current.scale,
                end: pop,
            },
        )
        .with_speed(**speed),
        Tween::new(
            EaseFunction::CircularOut,
            TweeningType::Once,
            Duration::from_millis(150),
            TransformScaleLens {
                start: pop,
                end: current.scale,
            },
        )
        .with_speed(**speed)
        .with_completed_event(true, AnimationEvent::COMPLETED.bits()),
    ]))
}

pub fn piece_loaded(
    index: u8,
    from: Transform,
    to: Transform,
    speed: &GameSpeed,
) -> Animator<Transform> {
    let bezier_6th = |x: f32| {
        let x2 = x * x;
        let x4 = x2 * x2;

        (-42.48f32).mul_add(
            x4 * x2,
            120.48f32.mul_add(
                x * x4,
                (-114f32).mul_add(x4, 34f32.mul_add(x * x2, 3. * x2)),
            ),
        )
    };
    let steady_velocity_time = (to.translation.x - from.translation.x).abs() * 16. / 1000.;
    let enter = Tween::new(
        EaseMethod::CustomFunction(bezier_6th),
        TweeningType::Once,
        Duration::from_secs_f32(steady_velocity_time),
        TransformPositionLens {
            start: from.translation,
            end: to.translation,
        },
    )
    .with_speed(**speed)
    .with_completed_event(true, AnimationEvent::COMPLETED.bits());

    if index > 0 {
        Animator::new(Sequence::new([
            Box::new(Delay::new(Duration::from_millis(10 * u64::from(index)))) as DynTweenable,
            Box::new(enter) as DynTweenable,
        ]))
    } else {
        Animator::new(enter)
    }
}

pub fn piece_movement(
    from: Transform,
    to: Transform,
    speed: &GameSpeed,
) -> RedoableAnimationBundle<Transform> {
    let animator = Animator::new(
        Tween::new(
            EaseFunction::CubicOut,
            TweeningType::Once,
            Duration::from_secs(3),
            TransformPositionLens {
                start: from.translation,
                end: to.translation,
            },
        )
        .with_speed(**speed)
        .with_completed_event(true, AnimationEvent::COMPLETED.bits()),
    );

    RedoableAnimationBundle {
        animator,
        original: Target(to),
    }
}

pub fn undo_selection(from: Transform, to: Transform, speed: &GameSpeed) -> Animator<Transform> {
    Animator::new(Tracks::new([
        Tween::new(
            EaseFunction::ExponentialInOut,
            TweeningType::Once,
            Duration::from_millis(250),
            TransformRotationLens {
                start: from.rotation,
                end: to.rotation,
            },
        )
        .with_speed(**speed),
        Tween::new(
            EaseFunction::ExponentialInOut,
            TweeningType::Once,
            Duration::from_millis(250),
            TransformPositionLens {
                start: from.translation,
                end: to.translation,
            },
        )
        .with_speed(**speed)
        .with_completed_event(true, AnimationEvent::COMPLETED.bits()),
    ]))
}

pub fn mouse_tutorial_enter(target: Transform, speed: &GameSpeed) -> Animator<Transform> {
    Animator::new(Tracks::new([
        Tween::new(
            EaseMethod::Linear,
            TweeningType::Once,
            Duration::from_millis(200),
            TransformPositionLens {
                start: target.translation - const_vec3!([-1., -1., 0.]),
                end: target.translation,
            },
        )
        .with_speed(**speed),
        Tween::new(
            EaseMethod::Linear,
            TweeningType::Once,
            Duration::from_millis(200),
            TransformScaleLens {
                start: Vec3::ZERO,
                end: target.scale,
            },
        )
        .with_speed(**speed)
        .with_completed_event(true, AnimationEvent::COMPLETED.bits()),
    ]))
}

pub fn mouse_tutorial_switch_rotation(
    from: Transform,
    to: Transform,
    speed: &GameSpeed,
    mirrored: bool,
) -> Animator<Transform> {
    Animator::new(Sequence::new([
        Box::new(Tracks::new([
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(150),
                TransformPositionLens {
                    start: from.translation,
                    end: from.translation
                        - if mirrored {
                            const_vec3!([-1.25, -1.25, 0.])
                        } else {
                            const_vec3!([-1.25, 1.25, 0.])
                        },
                },
            )
            .with_speed(**speed),
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(150),
                TransformScaleLens {
                    start: from.scale,
                    end: Vec3::ZERO,
                },
            )
            .with_speed(**speed),
        ])) as DynTweenable,
        Box::new(
            Tween::new(
                EaseMethod::Discrete(0.),
                TweeningType::Once,
                Duration::from_millis(25),
                TeleportLens(to.with_scale(Vec3::ZERO)),
            )
            .with_speed(**speed),
        ) as DynTweenable,
        Box::new(Tracks::new([
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(200),
                TransformPositionLens {
                    start: to.translation
                        - if mirrored {
                            const_vec3!([-1.25, 1.25, 0.])
                        } else {
                            const_vec3!([1.25, -1.25, 0.])
                        },
                    end: to.translation,
                },
            )
            .with_speed(**speed),
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(200),
                TransformScaleLens {
                    start: Vec3::ZERO,
                    end: to.scale,
                },
            )
            .with_speed(**speed)
            .with_completed_event(true, AnimationEvent::COMPLETED.bits()),
        ])) as DynTweenable,
    ]))
}

pub fn level_complete_menu_ui_enter(
    from: Rect<Val>,
    to: Rect<Val>,
    speed: &GameSpeed,
) -> Animator<Style> {
    Animator::new(
        Tween::new(
            EaseMethod::Linear,
            TweeningType::Once,
            Duration::from_millis(400),
            UiPositionLens {
                start: from,
                end: to,
            },
        )
        .with_speed(**speed)
        .with_completed_event(true, AnimationEvent::COMPLETED.bits()),
    )
}

pub fn level_complete_menu_ui_exit(
    from: Rect<Val>,
    to: Rect<Val>,
    speed: &GameSpeed,
) -> Animator<Style> {
    Animator::new(
        Tween::new(
            EaseMethod::Linear,
            TweeningType::Once,
            Duration::from_millis(300),
            UiPositionLens {
                start: from,
                end: to,
            },
        )
        .with_speed(**speed)
        .with_completed_event(
            true,
            (AnimationEvent::COMPLETED | AnimationEvent::OFFSCREEN).bits(),
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
                .remove::<Animator<T>>()
                .remove::<Target<T>>()
                .remove::<Original<T>>();
        }
    }
}

fn despawn_offscreen(
    mut commands: Commands,
    mut offscreen_animations: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, user_data } in offscreen_animations.iter() {
        if *user_data & AnimationEvent::OFFSCREEN.bits() != 0 {
            commands.entity(*entity).despawn_recursive();
        }
    }
}

fn ease_in_back(x: f32) -> f32 {
    const C1: f32 = 1.70158;
    const C3: f32 = C1 + 1.;

    let x2 = x * x;
    C3.mul_add(x * x2, -C1 * x2)
}

fn ease_out_back(x: f32) -> f32 {
    const C1: f32 = 1.70158;
    const C3: f32 = C1 + 1.;

    let x1 = x - 1.;
    let x2 = x1 * x1;
    1. + C3.mul_add(x1 * x2, C1 * x2)
}
