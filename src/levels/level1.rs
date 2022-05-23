use std::f32::consts::PI;

use bevy::{asset::LoadState, math::const_vec3, prelude::*};
use bevy_svg::prelude::{Origin, Svg, Svg2dBundle};
use bevy_tweening::Animator;

use crate::{
    animations,
    animations::GameSpeed,
    bags::{compute_bag_coordinates, BagContainerSpawner, BAG_ORIGIN},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    gb9000::GroceryBagger9000,
    levels::{transitions::LevelInitLabel, LevelMarker, LevelStarted},
    nominos::{
        Nomino, NominoMarker, NominoSpawner, PiecePickedUp, PiecePlaced, Selectable, DEG_90,
    },
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Gold;

pub struct Level1Plugin;

impl Plugin for Level1Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, init_level.label(LevelInitLabel));
        app.add_system(show_tutorial);
    }
}

fn init_level(
    mut commands: Commands,
    mut level_started: EventReader<LevelStarted>,
    mut placed_pieces: EventWriter<PiecePlaced>,
    dips_window: Res<DipsWindow>,
) {
    if !level_started.iter().last().map(|l| **l).contains(&0) {
        return;
    }

    commands.spawn_belt(Box::new(PresetPiecesConveyorBelt::new([Piece {
        nomino: Nomino::TetrominoL,
        color: LEVEL_COLOR,
        rotation: Quat::IDENTITY,
    }])));

    let bag = commands.spawn_bag::<1>(&dips_window)[0];

    // TODO use local coordinates after https://github.com/dimforge/bevy_rapier/issues/172
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelMarker)
        .with_children(|parent| {
            let origin = Transform::from_translation(
                compute_bag_coordinates(&dips_window, 1)[0] - BAG_ORIGIN,
            );
            macro_rules! spawn {
                ($nomino:expr, $transform:expr) => {{
                    parent
                        .spawn_nomino_into_bag(origin, $nomino, LEVEL_COLOR, $transform)
                        .id()
                }};
            }

            let pieces = [
                spawn!(Nomino::TetrominoSquare, Transform::from_xyz(0., 0., 0.)),
                spawn!(Nomino::TetrominoSquare, Transform::from_xyz(2., 0., 0.)),
                spawn!(Nomino::TetrominoSquare, Transform::from_xyz(4., 0., 0.)),
                spawn!(
                    Nomino::TetrominoL,
                    Transform::from_xyz(1., 3., 0.).with_rotation(*DEG_90)
                ),
                spawn!(
                    Nomino::TetrominoL,
                    Transform::from_xyz(2., 2., 0.).with_rotation(DEG_90.inverse())
                ),
                spawn!(Nomino::TetrominoSquare, Transform::from_xyz(4., 2., 0.)),
                spawn!(Nomino::TetrominoSquare, Transform::from_xyz(0., 4., 0.)),
                spawn!(
                    Nomino::TetrominoL,
                    Transform::from_xyz(4., 4., 0.).with_rotation(DEG_90.inverse())
                ),
            ];

            for piece in pieces {
                placed_pieces.send(PiecePlaced { piece, bag })
            }
        });
}

#[derive(Default)]
enum TutorialFsm {
    #[default]
    Ready,
    StartedLoad,
    Loading(Handle<Svg>, Entity),
    Loaded(Entity),
    PickedUp(Entity),
    Rotated,
}

#[derive(Component)]
struct TutorialIconMarker;

fn show_tutorial(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gb9000: ResMut<GroceryBagger9000>,
    game_speed: Res<GameSpeed>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut piece_selections: EventReader<PiecePickedUp>,
    mut fsm: Local<TutorialFsm>,
    first_piece: Query<
        Entity,
        (
            With<Selectable>,
            With<NominoMarker>,
            Without<Animator<Transform>>,
        ),
    >,
) {
    const ICON_SCALE: Vec3 = const_vec3!([0.05, 0.05, 0.05]);

    if gb9000.current_level != 0 {
        *fsm = TutorialFsm::Ready;
        return;
    }

    match &*fsm {
        TutorialFsm::Ready => {
            // TODO https://github.com/Weasy666/bevy_svg/issues/10
            *fsm = TutorialFsm::StartedLoad;
        }
        TutorialFsm::StartedLoad => {
            if let Ok(piece) = first_piece.get_single() {
                let handle = asset_server.load("icons/mouse-click.svg");
                commands.entity(piece).with_children(|parent| {
                    let transform =
                        Transform::from_translation(Vec3::new(-2., 0.5, 0.)).with_scale(Vec3::ZERO);

                    let icon = parent
                        .spawn_bundle(Svg2dBundle {
                            svg: handle.clone(),
                            transform,
                            origin: Origin::Center,
                            ..Default::default()
                        })
                        .insert(TutorialIconMarker)
                        .id();

                    *fsm = TutorialFsm::Loading(handle, icon);
                });
            }
        }
        TutorialFsm::Loading(handle, icon) => {
            if asset_server.get_load_state(handle) == LoadState::Loaded {
                commands
                    .entity(*icon)
                    .insert(animations::mouse_tutorial_enter(
                        Transform::from_scale(ICON_SCALE),
                        &game_speed,
                    ));

                *fsm = TutorialFsm::Loaded(*icon);
            }
        }
        TutorialFsm::Loaded(icon) => {
            if piece_selections.iter().count() > 0 {
                let transform =
                    Transform::from_translation(Vec3::new(-0.5, 1.5, 0.)).with_scale(ICON_SCALE);
                commands
                    .entity(*icon)
                    .insert(animations::mouse_tutorial_switch_rotation(
                        transform.with_rotation(Quat::from_rotation_y(PI)),
                        &game_speed,
                    ));

                *fsm = TutorialFsm::PickedUp(*icon);
            }
        }
        TutorialFsm::PickedUp(icon) => {
            if mouse_button_input.just_pressed(MouseButton::Right) {
                let transform =
                    Transform::from_translation(Vec3::new(-2., 0.5, 0.)).with_scale(ICON_SCALE);
                commands
                    .entity(*icon)
                    .insert(animations::mouse_tutorial_switch_rotation(
                        transform.with_rotation(DEG_90.inverse()),
                        &game_speed,
                    ));
                *fsm = TutorialFsm::Rotated;
            }
        }
        TutorialFsm::Rotated => {}
    }
}
