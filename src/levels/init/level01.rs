use bevy::{asset::LoadState, math::const_vec3, prelude::*};
use bevy_svg::prelude::{Svg, Svg2dBundle};
use bevy_tweening::Animator;

use crate::{
    animations,
    animations::GameSpeed,
    bags::{compute_bag_coordinates, BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    gb9000::{GameState::Playing, GroceryBagger9000},
    levels::{
        transitions::LevelSpawnStage,
        tutorials::{
            TUTORIAL_FONT_COLOR, TUTORIAL_FONT_SIZE_LARGE, TUTORIAL_FONT_SIZE_SMALL, TUTORIAL_STYLE,
        },
        LevelMarker, LevelStarted,
    },
    nominos::{
        Nomino, NominoMarker, NominoSpawner, PiecePickedUp, PiecePlaced, Selectable, DEG_180,
        DEG_90, DEG_MIRRORED,
    },
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Gold;

pub struct Level1Plugin;

impl Plugin for Level1Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            LevelSpawnStage,
            show_tutorial.after(super::super::init_levels),
        );
    }
}

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    placed_pieces: EventWriter<PiecePlaced>,
    asset_server: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    spawn_bag(&mut commands, &dips_window, placed_pieces);
    spawn_tutorial(&mut commands, asset_server);
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([Piece {
            nomino: Nomino::TetrominoStraight,
            color: LEVEL_COLOR,
            rotation: *DEG_90,
        }])),
    );
}

fn spawn_bag(
    commands: &mut Commands,
    dips_window: &DipsWindow,
    mut placed_pieces: EventWriter<PiecePlaced>,
) {
    let bag = commands.spawn_bag(dips_window, [BAG_SIZE_SMALL])[0];

    // TODO use local coordinates after https://github.com/dimforge/bevy_rapier/issues/172
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelMarker)
        .with_children(|parent| {
            let origin = Transform::from_translation(
                compute_bag_coordinates(dips_window, [BAG_SIZE_SMALL])[0] - BAG_SIZE_SMALL.origin(),
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
                spawn!(Nomino::TetrominoSquare, Transform::from_xyz(0., 2., 0.)),
            ];

            for piece in pieces {
                placed_pieces.send(PiecePlaced { piece, bag })
            }
        });
}

fn spawn_tutorial(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: TUTORIAL_STYLE(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(LevelMarker)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Welcome to Grocery Bagger 9000!",
                    TextStyle {
                        font: font.clone(),
                        font_size: TUTORIAL_FONT_SIZE_LARGE,
                        color: TUTORIAL_FONT_COLOR,
                    },
                    default(),
                ),
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(20.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Left click to pick up and place pieces\nRight click to rotate",
                    TextStyle {
                        font,
                        font_size: TUTORIAL_FONT_SIZE_SMALL,
                        color: TUTORIAL_FONT_COLOR,
                    },
                    default(),
                ),
                ..default()
            });
        });
}

#[derive(Default)]
enum TutorialFsm {
    #[default]
    Ready,
    StartedLoad,
    Loading(Handle<Svg>, Entity, Transform),
    Loaded(Entity, Transform),
    PickedUp(Entity, Transform),
    Rotated,
}

#[derive(Component)]
struct TutorialIconMarker;

fn show_tutorial(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gb9000: Res<GroceryBagger9000>,
    game_speed: Res<GameSpeed>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut level_started: EventReader<LevelStarted>,
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

    if gb9000.state != Playing || gb9000.current_level != 0 || level_started.iter().count() > 0 {
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
                    let transform = Transform::from_translation(const_vec3!([-1.25, -5., 0.]))
                        .with_scale(Vec3::ZERO)
                        .with_rotation(DEG_90.inverse());

                    let icon = parent
                        .spawn_bundle(Svg2dBundle {
                            svg: handle.clone(),
                            transform,
                            ..default()
                        })
                        .insert(TutorialIconMarker)
                        .id();

                    *fsm = TutorialFsm::Loading(handle, icon, transform);
                });
            }
        }
        TutorialFsm::Loading(handle, icon, from) => {
            if asset_server.get_load_state(handle) == LoadState::Loaded {
                let transform = from.with_scale(ICON_SCALE);
                commands
                    .entity(*icon)
                    .insert(animations::mouse_tutorial_enter(transform, &game_speed));

                *fsm = TutorialFsm::Loaded(*icon, transform);
            }
        }
        TutorialFsm::Loaded(icon, from) => {
            if piece_selections.iter().count() > 0 {
                let transform = Transform::from_translation(const_vec3!([-1.25, -2.75, 0.]))
                    .with_scale(ICON_SCALE)
                    .with_rotation(DEG_90.inverse() * *DEG_MIRRORED);
                commands
                    .entity(*icon)
                    .insert(animations::mouse_tutorial_switch_rotation(
                        *from,
                        transform,
                        &game_speed,
                        true,
                    ));

                *fsm = TutorialFsm::PickedUp(*icon, transform);
            }
        }
        TutorialFsm::PickedUp(icon, from) => {
            if mouse_button_input.just_pressed(MouseButton::Right) {
                let transform = Transform::from_translation(const_vec3!([1.25, -5., 0.]))
                    .with_scale(ICON_SCALE)
                    .with_rotation(*DEG_180);
                commands
                    .entity(*icon)
                    .insert(animations::mouse_tutorial_switch_rotation(
                        *from,
                        transform,
                        &game_speed,
                        false,
                    ));
                *fsm = TutorialFsm::Rotated;
            }
        }
        TutorialFsm::Rotated => {}
    }
}
