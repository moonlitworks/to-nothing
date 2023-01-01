use crate::state::GameState;
use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Startup).with_system(transition_to_splashscreen),
        );
    }
}

pub fn transition_to_splashscreen(mut state: ResMut<State<GameState>>) {
    if !state.is_added() {
        state.set(GameState::Splashscreen).unwrap();
    }
}
