use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{nominos::*, piece_movement::PieceMovementPlugin};

pub struct GroceryBagger9000Plugin;

impl Plugin for GroceryBagger9000Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(PieceMovementPlugin)
            .add_startup_system(setup);
    }
}

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
