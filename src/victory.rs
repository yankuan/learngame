use bevy::{
      color::palettes::css::{WHITE_SMOKE, YELLOW},
      prelude::*,
      window::PrimaryWindow,
  };
  
  use super::{
      assets::{FontKey, HandleMap},
      //game_time::GameTime,
      //ui::format_game_time,
      GameState,
  };

use crate::spawn::player::Respawn;
use crate::spawn::ball::Respawnball;
use crate::spawn::bricks::Respawnbricks;

  use crate::states::*;
  use crate::ui::prelude::*;
  //use rand::seq::SliceRandom;
  //use rand::thread_rng;
  
  pub(super) fn plugin(app: &mut App) {
      app.add_systems(OnEnter(GameState::Victory), init_ui)
          .add_systems(Update, handle_action.run_if(in_state(GameState::Victory)));
  }
  
  #[derive(Debug, Component)]
  struct StarSection;
  
  #[derive(Debug, Component)]
  struct TitleSection;
  
  #[derive(Debug, Component)]
  struct CommentsSection;
  
  #[derive(Debug, Component)]
  struct ArmComment;
  
  #[derive(Debug, Component)]
  struct TimeSection;
  
  #[derive(Debug, Component)]
  struct ButtonSection;
  
  #[derive(Debug, Component)]
  struct Star;
  
  #[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
  #[reflect(Component)]
  enum Action {
      Respawn,
      Title,
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
      //image_handles: Res<HandleMap<ImageKey>>,
      font_handles: Res<HandleMap<FontKey>>,
      q_window: Query<&Window, With<PrimaryWindow>>,
      //lost_limbs: Res<LostLimbs>,
      //game_time: Res<GameTime>,
  ) {
      //let mut rng = thread_rng();
      for window in &q_window {
          cmd.ui_center_root()
              .insert((
                  StateScoped(GameState::Victory),
                  BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
              ))
              .with_children(|cmd| {
                  cmd.spawn((
                      TitleSection,
                      TextBundle {
                          text: Text::from_section(
                              "Victory !",
                              TextStyle {
                                  font: font_handles[&FontKey::GeoFont].clone_weak(),
                                  font_size: window.height() / 10.,
                                  color: Color::from(WHITE_SMOKE),
                              },
                          ),
                          ..default()
                      },
                  ));
                  /* 
                  cmd.spawn((
                      TimeSection,
                      TextBundle {
                          text: Text::from_sections([
                              TextSection {
                                  value: "Time: ".to_string(),
                                  style: TextStyle {
                                      font: font_handles[&FontKey::GeoFont].clone_weak(),
                                      font_size: window.height() / 18.,
                                      color: Color::from(WHITE_SMOKE),
                                  },
                              },
                              TextSection {
                                  value: format_game_time(game_time.0),
                                  style: TextStyle {
                                      font: font_handles[&FontKey::GeoFont].clone_weak(),
                                      font_size: window.height() / 18.,
                                      color: Color::from(YELLOW),
                                  },
                              },
                          ]),
                          ..default()
                      },
                  ));
                  */
                  cmd.spawn((
                      CommentsSection,
                      NodeBundle {
                          style: Style {
                              flex_direction: FlexDirection::Column,
                              //width: Val::Vh(50.),
                              ..default()
                          },
                          //background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                          ..default()
                      },
                  ));
                  cmd.spawn(NodeBundle {
                      style: Style {
                          height: Val::Vh(5.),
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
                      let font_size = window.height() / 30.;
                      cmd.button(font_size, "Restart").insert(Action::Respawn);
                      //cmd.button(font_size, "Menu").insert(Action::Title);
                  });
              });
      }
  }
  