use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::{
    geometry::GeometryBuilder,
    prelude::{
        tess::{math::Point, path::path::Builder},
        DrawMode, FillMode, FillOptions, Path,
    },
};

use crate::{
    conveyor_belt::{
        consts::{
            BELT_NONSELECTABLE_BACKGROUND_COLOR, BELT_SELECTABLE_BACKGROUND_COLOR, PIECE_WIDTH,
            SELECTABLE_SEPARATION,
        },
        movement::BeltPieceIds,
        positioning::{compute_belt_position, compute_selectable_background},
        ConveyorBelt, HEIGHT, MAX_NUM_PIECES,
    },
    levels::LevelMarker,
    window_management::DipsWindow,
};

pub type BoxedConveyorBelt = Box<dyn ConveyorBelt + Send + Sync>;

#[derive(Component)]
pub struct ConveyorBeltMarker;

#[derive(Component)]
pub struct BeltSelectableBackgroundMarker;

#[derive(Component)]
pub struct BeltNonselectableBackgroundMarker;

#[derive(Component, Deref, DerefMut)]
pub struct ConveyorBeltInstance(BoxedConveyorBelt);

pub trait ConveyorBeltSpawner<'w, 's> {
    fn spawn_belt(
        &mut self,
        dips_window: &DipsWindow,
        belt: BoxedConveyorBelt,
    ) -> EntityCommands<'w, 's, '_>;
}

pub trait ConveyorBeltBackgroundSpawner {
    fn spawn_belt_background(&mut self, num_pieces_selectable: u8);
}

impl<'w, 's> ConveyorBeltSpawner<'w, 's> for Commands<'w, 's> {
    fn spawn_belt(
        &mut self,
        dips_window: &DipsWindow,
        belt: BoxedConveyorBelt,
    ) -> EntityCommands<'w, 's, '_> {
        let mut commands = self.spawn_bundle(TransformBundle::from_transform(
            compute_belt_position(dips_window),
        ));
        commands.insert(LevelMarker);
        commands.insert(ConveyorBeltInstance(belt));
        commands.insert(BeltPieceIds::default());
        commands.insert(ConveyorBeltMarker);
        commands
    }
}

impl<'w, 's, 'a> ConveyorBeltBackgroundSpawner for ChildBuilder<'w, 's, 'a> {
    fn spawn_belt_background(&mut self, num_pieces_selectable: u8) {
        let draw_mode = DrawMode::Fill(FillMode {
            options: FillOptions::default().with_intersections(false),
            color: BELT_SELECTABLE_BACKGROUND_COLOR,
        });

        self.spawn_bundle(GeometryBuilder::build_as(
            &selectable_background_path(num_pieces_selectable),
            draw_mode,
            Transform::default(),
        ))
        .insert(BeltSelectableBackgroundMarker);

        let draw_mode = DrawMode::Fill(FillMode {
            options: FillOptions::default().with_intersections(false),
            color: BELT_NONSELECTABLE_BACKGROUND_COLOR,
        });

        self.spawn_bundle(GeometryBuilder::build_as(
            &nonselectable_background_path(num_pieces_selectable),
            draw_mode,
            compute_selectable_background(num_pieces_selectable),
        ))
        .insert(BeltNonselectableBackgroundMarker);
    }
}

pub fn selectable_background_path(num_pieces_selectable: u8) -> Path {
    let selectable_width = compute_selectable_background(num_pieces_selectable)
        .translation
        .x;

    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(selectable_width, 0.));
    b.line_to(Point::new(selectable_width, HEIGHT));
    b.line_to(Point::new(0., HEIGHT));
    b.close();

    Path(b.build())
}

pub fn nonselectable_background_path(num_pieces_selectable: u8) -> Path {
    let selectable_width = f32::from(MAX_NUM_PIECES - num_pieces_selectable)
        .mul_add(PIECE_WIDTH, SELECTABLE_SEPARATION);

    let mut b = Builder::with_capacity(4, 5);

    b.begin(Point::new(0., 0.));
    b.line_to(Point::new(selectable_width, 0.));
    b.line_to(Point::new(selectable_width, HEIGHT));
    b.line_to(Point::new(0., HEIGHT));
    b.close();

    Path(b.build())
}
