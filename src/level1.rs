use bevy::prelude::*;

use crate::{
    bags,
    bags::{BagBundle, Level1Bag},
    conveyor_belt,
    dpi::{Dips, Pixels},
    levels::CurrentLevel,
    nominos::{NominoBundle, TetrominoL},
};

const LEVEL_COLOR: Color = Color::ORANGE;

pub struct Level1Plugin;

impl Plugin for Level1Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_level);
    }
}

#[derive(Deref)]
struct Level1Initialized {
    root: Entity,
}

fn init_level(
    mut commands: Commands,
    current: Res<CurrentLevel>,
    windows: Res<Windows>,
    initialized: Option<Res<Level1Initialized>>,
) {
    if current.level >= 1 {
        if let Some(initialized) = initialized {
            commands.entity(**initialized).despawn_recursive();
            commands.remove_resource::<Level1Initialized>();
        }
        return;
    } else if initialized.is_some() {
        return;
    }

    let root = commands
        .spawn_bundle(TransformBundle::default())
        .with_children(|parent| {
            let window = windows.get_primary().unwrap();
            let window_width = Pixels(window.width());
            let window_height = Pixels(window.height());

            let centered_bag_coords = Vec3::new(
                (*(window_width / Dips(2.) - bags::RADIUS)).round(),
                (*((window_height - conveyor_belt::HEIGHT) / Dips(2.) - bags::RADIUS)).round(),
                0.,
            );
            parent.spawn_bundle(BagBundle::new(
                Level1Bag::default(),
                LEVEL_COLOR,
                Transform::from_translation(centered_bag_coords),
            ));

            let l_position = Vec3::new(
                *(window_width - conveyor_belt::LENGTH),
                *(window_height - conveyor_belt::HEIGHT),
                0.,
            );
            parent.spawn_bundle(NominoBundle::new(
                TetrominoL::default(),
                LEVEL_COLOR,
                Transform::from_translation(l_position),
            ));
        })
        .id();
    commands.insert_resource(Level1Initialized { root });
}
