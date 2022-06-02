use bevy::{ecs::system::EntityCommands, math::const_vec3, prelude::*};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{FillMode, *},
};
use bevy_rapier3d::prelude::*;

use crate::{
    colors::NominoColor,
    nominos::{Nomino, NOMINO_COLLIDER_GROUP},
};

#[derive(Default, Component)]
pub struct NominoMarker;

pub trait NominoSpawner<'w, 's> {
    fn spawn_nomino_into_bag(
        &mut self,
        bag: Transform,
        nomino: Nomino,
        color: NominoColor,
        transform: Transform,
    ) -> EntityCommands<'w, 's, '_>;

    fn spawn_nomino(
        &mut self,
        position: Transform,
        nomino: Nomino,
        color: NominoColor,
        render_color: Color,
    ) -> EntityCommands<'w, 's, '_>;
}

#[derive(Bundle)]
pub struct NominoBundle {
    #[bundle]
    shape: ShapeBundle,
    nomino_marker: NominoMarker,
    nomino: Nomino,
    color: NominoColor,
}

impl NominoBundle {
    pub fn new(
        position: Transform,
        nomino: Nomino,
        color: NominoColor,
        render_color: Color,
    ) -> Self {
        let mut outline_color = color.render().as_hsla();
        if let Color::Hsla { lightness, .. } = &mut outline_color {
            *lightness = 0.28;
        } else {
            unreachable!()
        }

        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode {
                options: FillOptions::default().with_intersections(false),
                color: render_color,
            },
            outline_mode: StrokeMode::new(outline_color, 0.1),
        };

        Self {
            shape: GeometryBuilder::build_as(nomino.path(), draw_mode, position),
            nomino,
            color,
            nomino_marker: default(),
        }
    }
}

impl<'w, 's, 'a> NominoSpawner<'w, 's> for ChildBuilder<'w, 's, 'a> {
    fn spawn_nomino_into_bag(
        &mut self,
        bag: Transform,
        nomino: Nomino,
        color: NominoColor,
        mut transform: Transform,
    ) -> EntityCommands<'w, 's, '_> {
        // Offset by 0.5 since every piece is centered on a block
        transform.translation += bag.translation + const_vec3!([0.5, 0.5, 0.01]);
        transform.rotation *= bag.rotation;
        transform.scale *= bag.scale;

        self.spawn_nomino(transform, nomino, color, color.render())
    }

    fn spawn_nomino(
        &mut self,
        position: Transform,
        nomino: Nomino,
        color: NominoColor,
        render_color: Color,
    ) -> EntityCommands<'w, 's, '_> {
        let mut commands =
            self.spawn_bundle(NominoBundle::new(position, nomino, color, render_color));
        commands.insert(nomino.collider().clone());
        commands.insert(Sensor(true));
        commands.insert(NOMINO_COLLIDER_GROUP);
        commands
    }
}
