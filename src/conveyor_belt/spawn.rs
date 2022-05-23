use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    conveyor_belt::{movement::BeltPieceIds, ConveyorBelt},
    levels::LevelMarker,
};

pub type BoxedConveyorBelt = Box<dyn ConveyorBelt + Send + Sync>;

#[derive(Component)]
pub struct ConveyorBeltMarker;

#[derive(Component, Deref, DerefMut)]
pub struct ConveyorBeltInstance(BoxedConveyorBelt);

pub trait ConveyorBeltSpawner<'w, 's> {
    fn spawn_belt(&mut self, belt: BoxedConveyorBelt) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's> ConveyorBeltSpawner<'w, 's> for Commands<'w, 's> {
    fn spawn_belt(&mut self, belt: BoxedConveyorBelt) -> EntityCommands<'w, 's, '_> {
        let mut commands = self.spawn_bundle(TransformBundle::default());
        commands.insert(LevelMarker);
        commands.insert(ConveyorBeltInstance(belt));
        commands.insert(BeltPieceIds::default());
        commands.insert(ConveyorBeltMarker);
        commands
    }
}
