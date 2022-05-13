use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::conveyor_belt::{movement::BeltPieceIds, ConveyorBelt};

pub type BoxedConveyorBelt = Box<dyn ConveyorBelt + Send + Sync>;

#[derive(Component)]
pub struct ConveyorBeltMarker;

#[derive(Component, Deref, DerefMut)]
pub struct ConveyorBeltInstance(BoxedConveyorBelt);

pub trait ConveyorBeltSpawner<'w, 's> {
    fn spawn_belt(&mut self, belt: BoxedConveyorBelt) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's, 'a> ConveyorBeltSpawner<'w, 's> for ChildBuilder<'w, 's, 'a> {
    fn spawn_belt(&mut self, belt: BoxedConveyorBelt) -> EntityCommands<'w, 's, '_> {
        let mut commands = self.spawn();
        commands.insert(ConveyorBeltInstance(belt));
        commands.insert(BeltPieceIds::default());
        commands.insert(ConveyorBeltMarker);
        commands
    }
}
