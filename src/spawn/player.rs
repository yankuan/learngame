use bevy::{
      // color::palettes::css::*,
      math::{vec2, vec3},
      prelude::*,
};
use avian2d::{math::*, parry::shape::SharedShape, prelude::*};

use crate::{resource::Score, states::*};
use crate::componet::*;


pub(super) fn plugin(app: &mut App) {

      app.observe(on_spawn_player);
      app.observe(on_respawn);
    
}

#[derive(Event, Debug)]
pub struct Respawn;

#[derive(Debug, Event)]
pub struct Despawn;

#[derive(Event, Debug)]
pub struct SpawnPlayer(pub Vec3);

fn on_respawn(
      _trigger: Trigger<Respawn>,
      mut cmd: Commands,
) {
      //cmd.trigger(Despawn);
      cmd.trigger(SpawnPlayer(Vec3::new(0.,-74., 1.)));
}

fn on_spawn_player(
      trigger: Trigger<SpawnPlayer>,
      mut cmd: Commands,
      mut score: ResMut<Score>,
      mut players:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity),(With<player>,Without<wall>,Without<brick>)>,
) {
      let Ok((ent_player,mut trans, mut linear,mut ang)) = players.get_single_mut() else { return};
      trans.translation = trigger.event().0;
      **score = 0;
      //println!("{}",trigger.event().0);  
}
