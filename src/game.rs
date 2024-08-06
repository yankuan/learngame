use bevy::prelude::*;
use crate::states::GameState;
use avian2d::{math::*, parry::shape::SharedShape, prelude::*};

use crate::componet::*;
use crate::resource::*;

pub(super) fn plugin(app: &mut App) {
      app.add_sub_state::<GameState>();
      app.enable_state_scoped_entities::<GameState>();
      app.add_systems(Update, player_about.run_if(in_state(GameState::Playing)));
      app.add_systems(OnEnter(GameState::Death), player_stop);
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
      mut bricks:Query<(Entity),(With<brick>,Without<player>)>,
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
              println!("{:?}",*ballsta);
              if *ballsta == Ballstatus::Move {
                  linear_ball.y = 350.;   
              }
              //更具player的速度设置y线速度值
              //println!("{}",linear.x);
              //
          }
  
          for (ent_brick) in bricks.into_iter() {
              //println!("{}",ent_brick.index());
              if (entity1.index() == ent_ball.index()  && entity2.index() == ent_brick.index()) || (entity1.index() == ent_brick.index()  && entity2.index() == ent_ball.index())  {
                  commands.entity(ent_brick).despawn();
                  **score += 1;
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
          linear.x = -1. * 6000. * delta.delta_seconds();
          //trans.translation.x += -1. * 100. * delta.delta_seconds();
      }
      if keyboard.just_released(KeyCode::ArrowLeft) {
          linear.x = 0.;
      }
      if keyboard.pressed(KeyCode::ArrowRight) {
          linear.x = 1. * 6000. * delta.delta_seconds();
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
       
      for Collision(contacts) in collision_event_reader.read() {
          println!(
              "Entities {:?} and {:?} are colliding",
              contacts.entity1,
              contacts.entity2,
          );
      }
      
  }    
