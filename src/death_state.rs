use bevy::{color::palettes::css::WHITE_SMOKE, prelude::*, window::PrimaryWindow};


use super::assets::{FontKey, HandleMap};
  
use crate::states::*;
use crate::ui::prelude::*;


pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Death), init_ui);
}

#[derive(Debug, Component)]
struct TitleSection;

#[derive(Debug, Component)]
struct ButtonSection;

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
                    ));
                    /*
                    .with_children(|cmd| {
                        let font_size = window.height() / 35.;
                        cmd.button(font_size, "R for Respawn");
                        cmd.button(font_size, "Menu");
                    });
                    */
                });
        }

}