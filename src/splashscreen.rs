use crate::state::GameState;
use bevy::prelude::*;
use std::time::Duration;

pub struct SplashscreenPlugin;

#[derive(Component)]
pub struct Splashscreen;

#[derive(Resource)]
pub struct SplashscreenTimer(Timer);

impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SplashscreenTimer(Timer::default()))
            .add_systems(
                Update,
                update_splashscreen.run_if(in_state(GameState::Splashscreen)),
            )
            .add_systems(OnEnter(GameState::Splashscreen), add_splashscreen)
            .add_systems(OnExit(GameState::Splashscreen), remove_splashscreen);
    }
}

fn add_splashscreen(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut timer: ResMut<SplashscreenTimer>,
) {
    info!("Creating splashscreen");

    let texture = asset_server.load("cc-by-sa/moonlit-logo.png");

    commands
        .spawn((
            Splashscreen,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.075, 0.075, 0.075).into(),
                    custom_size: Some(Vec2::new(1280., 720.)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture,
                transform: Transform {
                    scale: Vec3::new(0.1, 0.1, 1.),
                    ..default()
                },
                ..default()
            });
        });

    timer.0 = Timer::new(Duration::from_secs(2), TimerMode::Once);
}

fn update_splashscreen(
    mut timer: ResMut<SplashscreenTimer>,
    mut state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if timer.0.just_finished() {
        state.set(GameState::MainMenu);
    } else {
        timer.0.tick(time.delta());
    }
}

fn remove_splashscreen(mut commands: Commands, q_splashscreen: Query<Entity, &Splashscreen>) {
    info!("Removing splashscreen");

    if let Some(splashscreen) = q_splashscreen.iter().next() {
        commands.entity(splashscreen).despawn_recursive();
    }
}
