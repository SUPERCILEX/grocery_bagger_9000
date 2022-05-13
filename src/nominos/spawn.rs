use bevy::{ecs::system::EntityCommands, math::const_vec3, prelude::*};
use bevy_prototype_lyon::prelude::{FillMode, *};
use bevy_rapier3d::prelude::*;

use crate::{
    colors::NominoColor,
    nominos::{Nomino, NOMINO_COLLIDER_GROUP},
};

#[derive(Component)]
pub struct NominoMarker;

pub trait NominoSpawner<'w, 's> {
    fn spawn_nomino(
        &mut self,
        bag: Transform,
        nomino: Nomino,
        color: NominoColor,
        transform: Transform,
    ) -> EntityCommands<'w, 's, '_>;

    fn spawn_nomino_with_color(
        &mut self,
        bag: Transform,
        nomino: Nomino,
        color: NominoColor,
        render_color: Color,
        transform: Transform,
    ) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's, 'a> NominoSpawner<'w, 's> for ChildBuilder<'w, 's, 'a> {
    fn spawn_nomino(
        &mut self,
        bag: Transform,
        nomino: Nomino,
        color: NominoColor,
        transform: Transform,
    ) -> EntityCommands<'w, 's, '_> {
        self.spawn_nomino_with_color(bag, nomino, color, color.render(), transform)
    }

    fn spawn_nomino_with_color(
        &mut self,
        bag: Transform,
        nomino: Nomino,
        color: NominoColor,
        render_color: Color,
        mut transform: Transform,
    ) -> EntityCommands<'w, 's, '_> {
        // Offset by 0.5 since every piece is centered on a block
        transform.translation += bag.translation + const_vec3!([0.5, 0.5, 0.01]);
        transform.rotation *= bag.rotation;
        transform.scale *= bag.scale;

        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode {
                options: FillOptions::default().with_intersections(false),
                color: render_color,
            },
            outline_mode: StrokeMode::new(Color::BLACK, 0.1),
        };

        let mut commands = self.spawn_bundle(GeometryBuilder::build_as(
            nomino.path(),
            draw_mode,
            transform,
        ));
        commands.insert(NominoMarker);
        commands.insert(nomino.collider().clone());
        commands.insert(Sensor(true));
        commands.insert(NOMINO_COLLIDER_GROUP);
        commands.insert(color);
        commands
    }
}
