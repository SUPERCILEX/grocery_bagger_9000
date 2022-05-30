use bevy::prelude::*;

use crate::{
    bags::{BagContainerSpawner, BAG_SIZE_SMALL},
    colors::NominoColor,
    conveyor_belt::{ConveyorBeltSpawner, Piece, PresetPiecesConveyorBelt},
    levels::tutorials::spawn_text_tutorial,
    nominos::{Nomino, NominoSpawner, DEG_90},
    window_management::DipsWindow,
};

const LEVEL_COLOR: NominoColor = NominoColor::Green;

pub fn init_level(
    mut commands: Commands,
    dips_window: Res<DipsWindow>,
    asset_server: Res<AssetServer>,
) {
    spawn_belt(&mut commands, &dips_window);
    spawn_bag(&mut commands, &dips_window);
    spawn_text_tutorial(
        &mut commands,
        asset_server,
        "Items sticking out of the bag\nare worth fewer points…",
    );
}

fn spawn_belt(commands: &mut Commands, dips_window: &DipsWindow) {
    commands.spawn_belt(
        dips_window,
        Box::new(PresetPiecesConveyorBelt::new([Piece {
            nomino: Nomino::TetrominoStraight,
            color: LEVEL_COLOR,
            rotation: Quat::IDENTITY,
        }])),
    );
}

fn spawn_bag(commands: &mut Commands, dips_window: &DipsWindow) {
    let bag = commands.spawn_bag(dips_window, [BAG_SIZE_SMALL])[0];

    commands.entity(bag).with_children(|parent| {
        let origin = Transform::from_translation(-BAG_SIZE_SMALL.origin());
        macro_rules! spawn {
            ($nomino:expr, $transform:expr) => {{
                parent
                    .spawn_nomino_into_bag(origin, $nomino, LEVEL_COLOR, $transform)
                    .id()
            }};
        }

        spawn!(
            Nomino::TetrominoL,
            Transform::from_xyz(1., 0., 0.).with_rotation(DEG_90.inverse())
        );
        spawn!(Nomino::TetrominoL, Transform::from_xyz(0., 2., 0.));
    });
}
