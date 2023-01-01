use crate::state::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenu;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(create_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(remove_menu));
    }
}

pub fn create_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("external/fonts/DePixelKlein.ttf");

    commands
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            let sections = "To Nothing"
                .chars()
                .enumerate()
                .map(|(index, char)| {
                    TextSection::new(
                        char,
                        TextStyle {
                            font: font.clone(),
                            font_size: 50.,
                            color: Color::Rgba {
                                red: 0.,
                                green: 0.,
                                blue: 0.,
                                alpha: 1. - (index as f32 * 0.08),
                            },
                        },
                    )
                })
                .collect::<Vec<_>>();

            parent.spawn(TextBundle {
                text: Text::from_sections(sections),
                ..default()
            });
        });
}

pub fn remove_menu() {}
