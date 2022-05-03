use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier3d::prelude::*;
use paste::paste;

use crate::nomino_consts::*;

pub const NOMINO_COLLIDER_GROUP: InteractionGroups = InteractionGroups::new(0b1, 0b1);

pub trait Nomino {
    fn path(&self) -> &Path;

    fn collider(&self) -> &ColliderShape;
}

#[derive(Bundle)]
struct NominoBundle {
    #[bundle]
    shape: ShapeBundle,
    #[bundle]
    collider: ColliderBundle,
}

pub trait NominoSpawner<'w, 's> {
    fn spawn_nomino(
        &mut self,
        bag: Transform,
        nomino: impl Nomino,
        color: Color,
        transform: Transform,
    ) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's, 'a> NominoSpawner<'w, 's> for ChildBuilder<'w, 's, 'a> {
    fn spawn_nomino(
        &mut self,
        base: Transform,
        nomino: impl Nomino,
        color: Color,
        mut transform: Transform,
    ) -> EntityCommands<'w, 's, '_> {
        // Offset by 0.5 since every piece is centered on a block
        transform.translation += base.translation + Vec3::new(0.5, 0.5, 0.);
        transform.rotation *= base.rotation;
        transform.scale *= base.scale;

        let collider = ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            shape: nomino.collider().clone().into(),
            position: (transform.translation, transform.rotation).into(),
            flags: ColliderFlags {
                collision_groups: NOMINO_COLLIDER_GROUP,
                ..default()
            }
            .into(),
            ..default()
        };

        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode {
                options: FillOptions::default().with_intersections(false),
                color,
            },
            outline_mode: StrokeMode::new(Color::BLACK, 0.1),
        };

        self.spawn_bundle(NominoBundle {
            shape: GeometryBuilder::build_as(nomino.path(), draw_mode, transform),
            collider,
        })
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

                fn collider(&self) -> &ColliderShape {
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
