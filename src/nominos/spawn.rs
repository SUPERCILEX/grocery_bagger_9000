use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::{FillMode, *};
use bevy_rapier3d::prelude::*;

use crate::{
    colors::NominoColor,
    nominos::{Nomino, NOMINO_COLLIDER_GROUP},
};

pub trait NominoSpawner<'w, 's> {
    fn spawn_nomino(
        &mut self,
        bag: Transform,
        nomino: Nomino,
        color: NominoColor,
        transform: Transform,
    ) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's, 'a> NominoSpawner<'w, 's> for ChildBuilder<'w, 's, 'a> {
    fn spawn_nomino(
        &mut self,
        base: Transform,
        nomino: Nomino,
        color: NominoColor,
        mut transform: Transform,
    ) -> EntityCommands<'w, 's, '_> {
        // Offset by 0.5 since every piece is centered on a block
        transform.translation += base.translation + Vec3::new(0.5, 0.5, 0.);
        transform.rotation *= base.rotation;
        transform.scale *= base.scale;

        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode {
                options: FillOptions::default().with_intersections(false),
                color: color.render(),
            },
            outline_mode: StrokeMode::new(Color::BLACK, 0.1),
        };

        let mut commands = self.spawn_bundle(GeometryBuilder::build_as(
            nomino.path(),
            draw_mode,
            transform,
        ));
        commands.insert(nomino.collider().clone());
        commands.insert(Sensor(true));
        commands.insert(NOMINO_COLLIDER_GROUP);
        commands.insert(color);
        commands
    }
}
