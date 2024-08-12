use bevy::prelude::*;
use std::time::Duration;
use crate::states::GameState;
use avian2d::{math::*, parry::shape::SharedShape, prelude::*};
use bevy::render::{
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    texture::BevyDefault,
    view::RenderLayers,
};
use bevy::sprite::Material2dPlugin;
use crate::game_time::GameTime;

use crate::componet::*;
use crate::resource::*;
use crate::manager;
use crate::mat::AnimationMaterial;
use crate::manager::*;


pub trait CameraLayer {
    const RENDER_LAYER: usize;

    fn layer() -> usize {
        Self::RENDER_LAYER
    }

    fn render_layers() -> RenderLayers {
        RenderLayers::from_layers(&[Self::RENDER_LAYER])
    }
}

#[derive(Component, Debug, Reflect, Default)]
pub struct BgSpriteCamera;
impl CameraLayer for BgSpriteCamera {
    const RENDER_LAYER: usize = 1;
}


#[derive(Component, Debug, Reflect, Default)]
pub struct SpriteCamera;
impl CameraLayer for SpriteCamera {
    const RENDER_LAYER: usize = 3;
}
#[derive(Resource, Debug, Copy, Clone, PartialEq)]
  pub enum BulletTime {
      Inactive,
      Active,
      Custom(f32),
  }
  impl BulletTime {
      pub fn factor(&self) -> f32 {
          match self {
              Self::Inactive => 1.0,
              Self::Active => 0.1,
              Self::Custom(val) => *val,
          }
      }
  }


pub(super) fn plugin(app: &mut App) {
      app.add_sub_state::<GameState>();
      app.enable_state_scoped_entities::<GameState>();
      app.add_systems(Update, (player_about).run_if(in_state(GameState::Wait)));
      app.add_systems(Update, (player_about,update_game_time).run_if(in_state(GameState::Playing)));
      app.add_systems(OnEnter(GameState::Death), player_stop);
      app.add_systems(OnEnter(GameState::Victory), player_stop);

      app.add_plugins(Material2dPlugin::<AnimationMaterial>::default());
      app.insert_resource(BulletTime::Inactive);
      manager::register_manager(app);    
}

fn update_game_time(
    mut q_game_time_text: Query<&mut Text, With<GameTimeText>>,
    game_time: Res<GameTime>,
) {
    for mut text in &mut q_game_time_text {
        text.sections[0].value = format_game_time(game_time.0);
    }
}
pub fn format_game_time(duration: Duration) -> String {
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    let millis = duration.subsec_millis();
    format!("{:0}:{:02}.{:02}", minutes, seconds, millis / 10)
}


fn player_stop(
      mut players:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity),(With<player>,Without<wall>,Without<brick>)>,
      mut balls:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity,&mut Ballstatus,&mut GravityScale),(With<ball>,Without<player>,Without<wall>,Without<brick>)>,
      mut state: ResMut<State<GameState>>
){ 
      if *state == GameState::Death {
      
            let Ok((ent_ball,mut trans_ball, mut linear_ball,mut ang_ball, mut ballsta, mut gs)) = balls.get_single_mut() else { return;};
            //let Ok((ent_checkover)) = checkovers.get_single_mut() else { return;};
            let Ok((ent_player,mut trans, mut linear,mut ang)) = players.get_single_mut() else { return};
            linear.x = 0.;
            linear_ball.x = 0.;
            ang_ball.0 = 0.;
      }
}

fn player_about(
      mut commands: Commands,
      keyboard:Res<ButtonInput<KeyCode>>,
      mut players:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity),(With<player>,Without<wall>,Without<brick>)>,
      mut balls:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity,&mut Ballstatus,&mut GravityScale),(With<ball>,Without<player>,Without<wall>,Without<brick>)>,
      mut bricks:Query<(Entity, &mut MultiAnimationManager),(With<brick>,Without<player>)>,
      mut checkovers:Query<(Entity),(With<checkover>)>,
      mut collision_event_reader: EventReader<Collision>,
      mut collision_event_reader_start: EventReader<CollisionStarted>,
      mut score: ResMut<Score>,
      mut next_state: ResMut<NextState<GameState>>,
      delta: Res<Time>,
  ){
      let Ok((ent_ball,mut trans_ball, mut linear_ball,mut ang_ball, mut ballsta, mut gs)) = balls.get_single_mut() else { return;};
      //let Ok((ent_checkover)) = checkovers.get_single_mut() else { return;};
      let Ok((ent_player,mut trans, mut linear,mut ang)) = players.get_single_mut() else { return};
      for CollisionStarted(entity1, entity2) in collision_event_reader_start.read() {
          if (entity1.index() == ent_ball.index()  && entity2.index() == ent_player.index()) || (entity1.index() == ent_player.index()  && entity2.index() == ent_ball.index())  {
              //println!("{}",entity1.index());
              //println!("{:?}",*ballsta);
              if *ballsta == Ballstatus::Move {
                  linear_ball.y = 350.;   
              }
              //更具player的速度设置y线速度值
              //println!("{}",linear.x);
              //
          }
  
          for (ent_brick,mut ma) in bricks.iter_mut() {
              //println!("{}",ent_brick.index());
              if (entity1.index() == ent_ball.index()  && entity2.index() == ent_brick.index()) || (entity1.index() == ent_brick.index()  && entity2.index() == ent_ball.index())  {
                  //
                  **score += 1;
                  if (**score == 11) {
                      next_state.set(GameState::Victory);
                  }
                  ma.manager_mut("core").reset_key_with_points("death", &mut commands);
                  //mul.manager_mut("core").reset_key_with_points("death", &mut commands);
                  //commands.entity(ent_brick).despawn_descendants();
              }
          }
  
          for (ent_checkover) in checkovers.into_iter() {
              if (entity1.index() == ent_ball.index()  && entity2.index() == ent_checkover.index()) || (entity1.index() == ent_checkover.index()  && entity2.index() == ent_ball.index())  {
                  //println!("{}",entity1.index());
                  println!("gameover");
                  
                  next_state.set(GameState::Death);
                  linear.x = 0.;
              }
          }     
      }
   

      //let (mut trans,mut linear) =  players.get_single_mut().expect("没有获取player实体");
  
      if keyboard.pressed(KeyCode::ArrowLeft) {
          linear.x = -1. * 10000. * delta.delta_seconds();
          //trans.translation.x += -1. * 100. * delta.delta_seconds();
      }
      if keyboard.just_released(KeyCode::ArrowLeft) {
          linear.x = 0.;
      }
      if keyboard.pressed(KeyCode::ArrowRight) {
          linear.x = 1. * 10000. * delta.delta_seconds();
          //trans.translation.x += 1. * 100. * delta.delta_seconds();  
      }
      if keyboard.just_released(KeyCode::ArrowRight) {
          linear.x = 0.;
      }
  
      if *ballsta == Ballstatus::Nomove {
          linear_ball.x = linear.x;
      }
  
  
      if keyboard.pressed(KeyCode::ArrowUp) {
          //linear.y = 1. * 5000. * delta.delta_seconds();
          //trans.translation.y += 1. * 100. * delta.delta_seconds();
          if *ballsta == Ballstatus::Nomove {
              linear_ball.y = 350.;
              gs.0 = 20.;
              *ballsta = Ballstatus::Move;
              next_state.set(GameState::Playing);
              
          }
      }
      /*
      if keyboard.pressed(KeyCode::ArrowDown) {
          //linear.y = -1. * 5000. * delta.delta_seconds();
          trans.translation.y += -1. * 100. * delta.delta_seconds();
      }
      */
      if trans.translation.x <= -112. {
          linear.x = 0.;
          trans.translation.x = -111.;
      }
      if trans.translation.x > 112. {
          linear.x = 0.;
          trans.translation.x = 111.;
      }
  }    
