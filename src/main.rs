mod main_menu;
mod splashscreen;
mod state;

use bevy::{prelude::*, window::*};
use state::GameState;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.075, 0.075, 0.075)))
        .add_state::<GameState>()
        .add_plugins((
            splashscreen::SplashscreenPlugin,
            main_menu::MainMenuPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                ..default()
            }),
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(PostStartup, setup_primary_window)
        .add_systems(PostStartup, start_splashscreen)
        .run();
}

fn setup_camera(mut commands: Commands) {
    info!("Creating camera");

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.33,
            ..default()
        },
        ..default()
    });
}

fn setup_primary_window(mut commands: Commands) {
    info!("Creating primary window");

    commands.spawn((
        PrimaryWindow,
        Window {
            resolution: (1280., 720.).into(),
            title: "To Nothing".to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoVsync,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        },
    ));
}

fn start_splashscreen(mut state: ResMut<NextState<GameState>>) {
    info!("Starting splashscreen");

    state.set(GameState::Splashscreen);
}
