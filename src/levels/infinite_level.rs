use bevy::prelude::*;

use crate::{
    animations::GameSpeed,
    bags::{BagContainerSpawner, BagSize, BAG_SIZE_LARGE, BAG_SIZE_SMALL},
    conveyor_belt::{ConveyorBeltSpawner, InfinitePiecesConveyorBelt},
    nominos::NominoColor,
    robot::RobotSpawner,
    window_management::DipsWindow,
};

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    game_speed: Res<GameSpeed>,
) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(
        &dips_window,
        &game_speed,
        &[
            BAG_SIZE_LARGE,
            BAG_SIZE_SMALL,
            BagSize::new(4, 2),
            BAG_SIZE_SMALL,
            BAG_SIZE_LARGE,
        ],
    );
    commands.spawn_robot();
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(InfinitePiecesConveyorBelt::new([
            NominoColor::Orange,
            NominoColor::Gold,
            NominoColor::Green,
            NominoColor::Pink,
        ])),
    );
}
