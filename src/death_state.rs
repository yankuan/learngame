use bevy::{color::palettes::css::WHITE_SMOKE, prelude::*, window::PrimaryWindow};

use bevy::prelude::*;

use super::assets::{FontKey, HandleMap};
  
use crate::states::*;
use crate::ui::prelude::*;
use crate::spawn::player::Respawn;
use crate::spawn::ball::Respawnball;
use crate::spawn::bricks::Respawnbricks;

#[derive(Debug, Component)]
struct TitleSection;

#[derive(Debug, Component)]
struct ButtonSection;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum Action {
    Respawn,
    Title,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Death), init_ui);
    app.add_systems(
        Update,
        handle_action.run_if(in_state(GameState::Death)),
    );
}

fn handle_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut button_query: InteractionQuery<&Action>,
    mut cmd: Commands,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                Action::Respawn => {
                    next_game_state.set(GameState::Playing);
                    cmd.trigger(Respawn);
                    cmd.trigger(Respawnball);
                    cmd.trigger(Respawnbricks);
                }
                Action::Title => {
                    next_screen.set(Screen::Title);
                }
            }
        }
    }
}



fn init_ui(
      mut cmd: Commands,
      font_handles: Res<HandleMap<FontKey>>,
      q_window: Query<&Window, With<PrimaryWindow>>,
) {
      for window in &q_window {
            cmd.ui_center_root()
                .insert((
                    StateScoped(GameState::Death),
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                ))
                .with_children(|cmd| {
                    cmd.spawn((
                        TitleSection,
                        TextBundle {
                            text: Text::from_section(
                                "Game Over",
                                TextStyle {
                                    font: font_handles[&FontKey::GeoFont].clone_weak(),
                                    font_size: window.height() / 8.,
                                    color: Color::from(WHITE_SMOKE),
                                },
                            ),
                            ..default()
                        },
                    ));
                    cmd.spawn(NodeBundle {
                        style: Style {
                            height: Val::Vh(10.),
                            ..default()
                        },
                        ..default()
                    });
                    cmd.spawn((
                        ButtonSection,
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Vh(5.),
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .with_children(|cmd| {
                        let font_size = window.height() / 35.;
                        cmd.button(font_size, "Respawn").insert(Action::Respawn);
                    });
                    
                });
        }

}