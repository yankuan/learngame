use bevy::{
      // color::palettes::css::*,
      math::{vec2, vec3},
      prelude::*,
};
use avian2d::{math::*, parry::shape::SharedShape, prelude::*};

use crate::states::*;
use crate::componet::*;


pub(super) fn plugin(app: &mut App) {

      app.observe(on_spawn_ball);
      app.observe(on_respawn);
    
}

#[derive(Event, Debug)]
pub struct Respawnball;

#[derive(Debug, Event)]
pub struct Despawnball;

#[derive(Event, Debug)]
pub struct SpawnBall(pub Vec3);

fn on_respawn(
      _trigger: Trigger<Respawnball>,
      mut cmd: Commands,
) {
      //cmd.trigger(Despawn);
      cmd.trigger(SpawnBall(Vec3::new(0.,-64., 1.)));
}

fn on_spawn_ball(
      trigger: Trigger<SpawnBall>,
      mut cmd: Commands,
      mut balls:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity,&mut Ballstatus,&mut GravityScale),(With<ball>,Without<player>,Without<wall>,Without<brick>)>,
) {
      let Ok((ent_ball,mut trans_ball, mut linear_ball,mut ang_ball, mut ballsta, mut gs)) = balls.get_single_mut() else { return;};
      trans_ball.translation = trigger.event().0;
      *ballsta = Ballstatus::Nomove;
      //println!("{}",trigger.event().0);  
}