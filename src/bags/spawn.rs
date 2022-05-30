use bevy::{ecs::system::EntityCommands, math::const_vec3, prelude::*};
use bevy_prototype_lyon::prelude::{
    tess::{geom::Point, path::path::Builder},
    FillMode, *,
};
use bevy_rapier3d::prelude::*;
use smallvec::SmallVec;

use crate::{
    animations,
    animations::GameSpeed,
    bags::{bag_size::BagSize, consts::*, positioning::compute_container_coordinates},
    levels::LevelMarker,
    window_management::DipsWindow,
};

#[derive(Component)]
pub struct BagContainerMarker;

#[derive(Component)]
pub struct BagMarker;

#[derive(Component)]
pub struct BagLidMarker;

#[derive(Component)]
pub struct BagWallsMarker;

#[derive(Component)]
pub struct BagFloorMarker;

pub trait BagContainerSpawner {
    fn spawn_bag(
        &mut self,
        window: &DipsWindow,
        game_speed: &GameSpeed,
        sizes: impl IntoIterator<Item = BagSize> + Copy,
    ) -> SmallVec<[Entity; 3]>;
}

pub trait BagSpawner<'w, 's> {
    fn spawn_replacement_bag(
        &mut self,
        game_speed: &GameSpeed,
        transform: Transform,
        bag_size: BagSize,
    ) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's> BagContainerSpawner for Commands<'w, 's> {
    fn spawn_bag(
        &mut self,
        window: &DipsWindow,
        game_speed: &GameSpeed,
        sizes: impl IntoIterator<Item = BagSize> + Copy,
    ) -> SmallVec<[Entity; 3]> {
        let mut spawned_bags = SmallVec::new();

        let base = compute_container_coordinates(window, sizes);
        self.spawn_bundle(TransformBundle::from(Transform::from_translation(base)))
            .insert(LevelMarker)
            .insert(BagContainerMarker)
            .with_children(|parent| {
                let mut starting_position = Vec3::ZERO;
                for size in sizes {
                    starting_position.x += size.half_width();
                    spawned_bags.push(
                        spawn_bag(
                            parent,
                            game_speed,
                            Transform::from_translation(starting_position),
                            size,
                            false,
                        )
                        .id(),
                    );
                    starting_position.x += size.half_width() + f32::from(BAG_SPACING);
                }
            });

        spawned_bags
    }
}

impl<'w, 's, 'a> BagSpawner<'w, 's> for ChildBuilder<'w, 's, 'a> {
    fn spawn_replacement_bag(
        &mut self,
        game_speed: &GameSpeed,
        transform: Transform,
        bag_size: BagSize,
    ) -> EntityCommands<'w, 's, '_> {
        spawn_bag(self, game_speed, transform, bag_size, true)
    }
}

fn spawn_bag<'w, 's, 'a>(
    commands: &'a mut ChildBuilder<'w, 's, '_>,
    game_speed: &GameSpeed,
    transform: Transform,
    bag_size: BagSize,
    is_replacement: bool,
) -> EntityCommands<'w, 's, 'a> {
    let draw_mode = DrawMode::Outlined {
        fill_mode: FillMode {
            options: FillOptions::default().with_intersections(false),
            color: BAG_COLOR,
        },
        outline_mode: StrokeMode::new(BAG_OUTLINE_COLOR, 0.15),
    };

    let entry_transform = transform.with_scale(Vec3::ZERO);
    let mut commands = commands.spawn_bundle(GeometryBuilder::build_as(
        &bag_path(bag_size),
        draw_mode,
        entry_transform,
    ));
    commands.insert(BagMarker);
    commands.insert(bag_size);
    commands.insert(animations::bag_enter(
        entry_transform,
        transform,
        game_speed,
        is_replacement,
    ));

    commands.insert(Collider::cuboid(
        bag_size.half_width(),
        bag_size.half_height(),
        0.,
    ));
    commands.insert(Sensor(true));
    commands.insert(BAG_COLLIDER_GROUP);
    commands.with_children(|parent| {
        parent
            .spawn_bundle(TransformBundle::default())
            .insert(BagLidMarker)
            .insert(Collider::compound(vec![(
                const_vec3!([0., bag_size.half_height() + LID_OFFSET, 0.]),
                Quat::IDENTITY,
                Collider::cuboid(bag_size.half_width(), LID_HALFHEIGHT, 0.),
            )]))
            .insert(Sensor(true))
            .insert(BAG_LID_COLLIDER_GROUP);

        parent
            .spawn_bundle(TransformBundle::default())
            .insert(BagWallsMarker)
            .insert(Collider::compound(vec![
                (
                    const_vec3!([-bag_size.half_width(), 0., 0.]),
                    Quat::IDENTITY,
                    Collider::cuboid(BOUNDARY_HALFWIDTH, bag_size.half_height(), 0.),
                ),
                (
                    const_vec3!([bag_size.half_width(), 0., 0.]),
                    Quat::IDENTITY,
                    Collider::cuboid(BOUNDARY_HALFWIDTH, bag_size.half_height(), 0.),
                ),
            ]))
            .insert(Sensor(true))
            .insert(BAG_WALLS_COLLIDER_GROUP);

        parent
            .spawn_bundle(TransformBundle::default())
            .insert(BagFloorMarker)
            .insert(Collider::compound(vec![(
                const_vec3!([0., -bag_size.half_height(), 0.]),
                Quat::IDENTITY,
                Collider::cuboid(bag_size.half_width(), BOUNDARY_HALFWIDTH, 0.),
            )]))
            .insert(Sensor(true))
            .insert(BAG_FLOOR_COLLIDER_GROUP);
    });
    commands
}

fn bag_path(bag_size: BagSize) -> Path {
    let half_width = bag_size.half_width() as f32;
    let half_height = bag_size.half_height() as f32;

    let mut b = Builder::with_capacity(4, 4);

    b.begin(Point::new(-half_width, half_height));
    b.line_to(Point::new(-half_width, -half_height));
    b.line_to(Point::new(half_width, -half_height));
    b.line_to(Point::new(half_width, half_height));
    b.end(false);

    Path(b.build())
}
