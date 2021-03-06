use std::{f32::consts::PI, time::Duration};

use bevy::{ecs::schedule::ShouldRun, math::const_vec3, prelude::*};
use bevy_tweening::{
    lens::{
        TextColorLens, TransformPositionLens, TransformRotationLens, TransformScaleLens,
        UiPositionLens,
    },
    *,
};
use bitflags::bitflags;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSpeed>();

        macro_rules! add_change_animation_speed {
            ($t:ty) => {
                app.add_system(
                    change_animation_speed::<$t>
                        .with_run_criteria(run_if_game_speed_changed)
                        .before(AnimationSystem::AnimationUpdate),
                );
            };
        }

        add_change_animation_speed!(Transform);
        add_change_animation_speed!(Style);
        add_change_animation_speed!(Text);

        macro_rules! add_cleanup_animations {
            ($t:ty) => {
                app.add_system(cleanup_animations::<$t>.after(AnimationSystem::AnimationUpdate));
            };
        }

        add_cleanup_animations!(Transform);
        add_cleanup_animations!(Style);
        add_cleanup_animations!(Text);

        app.add_system(handle_animation_despawns.after(AnimationSystem::AnimationUpdate));
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
    target: Target<T>,
}

#[derive(Bundle)]
pub struct AnimationComponentsBundle<T: Component> {
    animator: Animator<T>,
    original: Original<T>,
    target: Target<T>,
}

bitflags! {
    pub struct AnimationEvent: u64 {
        const COMPLETED = 1;
        const DESPAWNABLE = 1 << 1;
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

pub fn bag_enter(
    from: Transform,
    to: Transform,
    speed: &GameSpeed,
    is_replacement: bool,
) -> Animator<Transform> {
    Animator::new(Sequence::new([
        Box::new(Delay::new(Duration::from_millis(if is_replacement {
            500
        } else {
            100
        }))) as DynTweenable,
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
            (AnimationEvent::COMPLETED | AnimationEvent::DESPAWNABLE | AnimationEvent::BAG).bits(),
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

pub fn score_particle(
    from: GlobalTransform,
    to: GlobalTransform,
    from_color: Color,
    to_color: Color,
    speed: &GameSpeed,
) -> (Animator<Transform>, Animator<Text>) {
    // TODO remove after new animation code
    type DynTweenable = Box<dyn Tweenable<Text> + Send + Sync + 'static>;

    let enter = Animator::new(
        Tween::new(
            EaseFunction::QuinticOut,
            TweeningType::Once,
            Duration::from_millis(600),
            TransformPositionLens {
                start: from.translation,
                end: to.translation,
            },
        )
        .with_speed(**speed),
    );

    let exit = Animator::new(Sequence::new([
        Box::new(Delay::new(Duration::from_millis(200))) as DynTweenable,
        Box::new(
            Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_millis(600),
                TextColorLens {
                    start: from_color,
                    end: to_color,
                    section: 0,
                },
            )
            .with_speed(**speed)
            .with_completed_event(
                true,
                (AnimationEvent::COMPLETED | AnimationEvent::DESPAWNABLE).bits(),
            ),
        ) as DynTweenable,
    ]));

    (enter, exit)
}

pub fn piece_loaded(
    index: u8,
    from: Transform,
    to: Transform,
    speed: &GameSpeed,
) -> Animator<Transform> {
    fn bezier_6th(x: f32) -> f32 {
        let x2 = x * x;
        let x4 = x2 * x2;

        (-42.48f32).mul_add(
            x4 * x2,
            120.48f32.mul_add(
                x * x4,
                (-114f32).mul_add(x4, 34f32.mul_add(x * x2, 3. * x2)),
            ),
        )
    }

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
        target: Target(to),
    }
}

pub fn undo_selection(
    from: Transform,
    to: Transform,
    speed: &GameSpeed,
) -> RedoableAnimationBundle<Transform> {
    let animator = Animator::new(Tracks::new([
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
    ]));

    RedoableAnimationBundle {
        animator,
        target: Target(to),
    }
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
            (AnimationEvent::COMPLETED | AnimationEvent::DESPAWNABLE).bits(),
        ),
    )
}

fn run_if_game_speed_changed(game_speed: Res<GameSpeed>) -> ShouldRun {
    if game_speed.is_changed() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn change_animation_speed<T: Component>(
    game_speed: Res<GameSpeed>,
    mut animators: Query<&mut Animator<T>>,
) {
    for mut animator in animators.iter_mut() {
        animator.set_speed(**game_speed);
    }
}

fn cleanup_animations<T: Component>(
    mut commands: Commands,
    mut completed_animations: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, user_data } in completed_animations.iter() {
        let flags = AnimationEvent::COMPLETED.bits();
        if *user_data & flags == flags {
            commands
                .entity(*entity)
                .remove_bundle::<AnimationComponentsBundle<T>>();
        }
    }
}

fn handle_animation_despawns(
    mut commands: Commands,
    mut animations_despawns: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, user_data } in animations_despawns.iter() {
        let flags = AnimationEvent::DESPAWNABLE.bits();
        if *user_data & flags == flags {
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
