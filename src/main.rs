mod main_menu;
mod splashscreen;
mod startup;
mod state;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.075, 0.075, 0.075).into()))
        .add_plugin(startup::StartupPlugin)
        .add_plugin(splashscreen::SplashscreenPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_state(state::GameState::Startup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1280.,
                height: 720.,
                title: "To Nothing".to_string(),
                resizable: false,
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.33,
            ..default()
        },
        ..default()
    });
}
