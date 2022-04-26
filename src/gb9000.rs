use bevy::{ecs::system::EntityCommands, prelude::*, render::camera::RenderTarget};
use bevy_prototype_lyon::prelude::*;

use crate::{nominos::*, window_management::MainCamera};

pub struct GroceryBagger9000Plugin;

impl Plugin for GroceryBagger9000Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(piece_selection_handler)
            .add_system(selected_piece_mover.before(piece_selection_handler))
            .init_resource::<PieceSelection>();
    }
}

#[derive(Deref, Default)]
struct PieceSelection(Option<SelectedPiece>);

struct SelectedPiece {
    id: Entity,
    offset: Vec2,
}

#[derive(Component)]
struct PieceSelectedMarker;

fn setup(mut commands: Commands) {
    spawn_nomino(
        &mut commands,
        TetrominoStraight::default(),
        Color::RED,
        Transform::from_translation(Vec3::new(2., 10., 0.)),
    );
    spawn_nomino(
        &mut commands,
        TetrominoSquare::default(),
        Color::ORANGE,
        Transform::from_translation(Vec3::new(7., 10., 0.)),
    );
    spawn_nomino(
        &mut commands,
        TetrominoT::default(),
        Color::CYAN,
        Transform::from_translation(Vec3::new(10., 10., 0.)),
    );
    spawn_nomino(
        &mut commands,
        TetrominoL::default(),
        Color::GREEN,
        Transform::from_translation(Vec3::new(14., 10., 0.)),
    );
    spawn_nomino(
        &mut commands,
        TetrominoSkew::default(),
        Color::FUCHSIA,
        Transform::from_translation(Vec3::new(17., 10., 0.)),
    );
}

fn piece_selection_handler(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    pieces: Query<(Entity, &Transform, &BoundingBoxes), With<NominoMarker>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut selected_piece: ResMut<PieceSelection>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if selected_piece.is_some() {
            commands
                .entity((*selected_piece).as_ref().unwrap().id)
                .remove::<PieceSelectedMarker>();
            *selected_piece = default();
            return;
        }

        if let Some(cursor_position) = compute_cursor_position(windows, camera) {
            for (id, transform, bounds) in pieces.iter() {
                if bounds.contains(transform, cursor_position) {
                    commands.entity(id).insert(PieceSelectedMarker);
                    *selected_piece = PieceSelection(Some(SelectedPiece {
                        id,
                        offset: cursor_position - transform.translation.truncate(),
                    }));
                }
            }
        }
    }
}

fn selected_piece_mover(
    selected_piece: Res<PieceSelection>,
    mut mouse_movements: EventReader<CursorMoved>,
    mut position: Query<&mut Transform, With<PieceSelectedMarker>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Some(selected_piece) = (*selected_piece).as_ref() {
        let (camera, camera_transform) = camera.single();
        let mut position = position.single_mut();
        for e in mouse_movements.iter() {
            position.translation = (window_to_world_coords(
                e.position,
                windows.get(e.id).unwrap(),
                camera,
                camera_transform,
            ) - selected_piece.offset)
                .extend(0.);
        }
    }
}

fn compute_cursor_position(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera.single();
    let window = get_main_window(&windows, camera);

    window
        .cursor_position()
        .map(|position| window_to_world_coords(position, window, camera, camera_transform))
}

fn window_to_world_coords(
    coords: Vec2,
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (coords / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    world_pos.truncate()
}

fn get_main_window<'a>(windows: &'a Res<Windows>, camera: &Camera) -> &'a Window {
    if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    }
}

fn spawn_nomino<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    nomino: impl Nomino,
    fill_color: Color,
    transform: Transform,
) -> EntityCommands<'w, 's, 'a> {
    let draw_mode = DrawMode::Outlined {
        fill_mode: FillMode {
            options: FillOptions::default().with_intersections(false),
            color: fill_color,
        },
        outline_mode: StrokeMode::new(Color::BLACK, 0.15),
    };

    commands.spawn_bundle(NominoBundle::new(nomino, draw_mode, transform))
}
