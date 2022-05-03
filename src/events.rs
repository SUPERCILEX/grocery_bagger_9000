use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PiecePlaced>();
    }
}

pub struct PiecePlaced {
    pub piece: Entity,
    pub bag: Entity,
}
