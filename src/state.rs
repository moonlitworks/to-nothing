use bevy::prelude::States;

#[allow(dead_code)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Startup,
    Splashscreen,
    MainMenu,
    Dialogue,
    Freemove,
}
