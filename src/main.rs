mod main_menu;
mod splashscreen;
mod state;

use bevy::prelude::*;
use state::GameState;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.075, 0.075, 0.075)))
        .add_state::<GameState>()
        .add_plugins((
            splashscreen::SplashscreenPlugin,
            main_menu::MainMenuPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1280., 720.).into(),
                    title: "To Nothing".to_string(),
                    resizable: false,
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands, mut state: ResMut<NextState<GameState>>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.33,
            ..default()
        },
        ..default()
    });

    state.set(GameState::Splashscreen);
}
