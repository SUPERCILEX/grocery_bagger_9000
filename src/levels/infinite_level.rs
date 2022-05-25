use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BagSize, BAG_SIZE_LARGE, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, InfinitePiecesConveyorBelt},
    window_management::DipsWindow,
};

pub fn init_level(mut commands: Commands, dips_window: Res<DipsWindow>) {
    spawn_belt(&mut commands, &dips_window);
    commands.spawn_bag(
        &dips_window,
        [
            BAG_SIZE_LARGE,
            BAG_SIZE_SMALL,
            BagSize::new(4, 2),
            BAG_SIZE_SMALL,
            BAG_SIZE_LARGE,
        ],
    );
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(InfinitePiecesConveyorBelt::new([
            NominoColor::Red,
            NominoColor::Gold,
            NominoColor::Blue,
            NominoColor::Green,
            NominoColor::Pink,
        ])),
    );
}
