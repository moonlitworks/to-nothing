use bevy::prelude::*;

pub struct FadePlugin;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum FadeState {
    In,
    Out,
    None,
}

#[derive(Component)]
pub struct Fade {
    timer: Timer,
    color: Color,
}

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(FadeState::None).with_system(clear_fade))
            .add_system(fade_event_listener);
    }
}

fn clear_fade(mut commands: Commands, q_fade: Query<(Entity, &Fade)>) {}

fn fade_event_listener() {}
