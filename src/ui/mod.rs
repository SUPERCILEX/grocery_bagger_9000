use bevy::prelude::*;

use consts::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use hud::HudPlugin;
use in_game_menu::InGameMenuPlugin;
use level_end_menu::LevelEndMenuPlugin;

mod consts;
mod hud;
mod in_game_menu;
mod level_end_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HudPlugin);
        app.add_plugin(LevelEndMenuPlugin);
        app.add_plugin(InGameMenuPlugin);

        app.add_system(button_hover_system);
    }
}

fn button_hover_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        *color = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
        .into();
    }
}
