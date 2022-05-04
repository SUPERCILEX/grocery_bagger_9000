use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::{FillMode, *};
use bevy_rapier3d::prelude::*;
use paste::paste;

use crate::{colors::NominoColor, nomino_consts::*};

pub const NOMINO_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b1,
    filters: 0b1,
};

pub trait Nomino {
    fn path(&self) -> &Path;

    fn collider(&self) -> &Collider;
}

pub trait NominoSpawner<'w, 's> {
    fn spawn_nomino(
        &mut self,
        bag: Transform,
        nomino: impl Nomino,
        color: NominoColor,
        transform: Transform,
    ) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's, 'a> NominoSpawner<'w, 's> for ChildBuilder<'w, 's, 'a> {
    fn spawn_nomino(
        &mut self,
        base: Transform,
        nomino: impl Nomino,
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

macro_rules! nomino {
    ($type:ident, $shape:ident) => {
        paste! {
            #[derive(Default)]
            pub struct [<$type $shape>];

            impl Nomino for [<$type $shape>] {
                fn path(&self) -> &Path {
                    &[<$type:upper _ $shape:upper _PATH>]
                }

                fn collider(&self) -> &Collider {
                    &*[<$type:upper _ $shape:upper _COLLIDER>]
                }
            }
        }
    };
}

nomino!(Tetromino, Straight);
nomino!(Tetromino, Square);
nomino!(Tetromino, T);
nomino!(Tetromino, L);
nomino!(Tetromino, Skew);
