use bevy::{
      // color::palettes::css::*,
      math::{vec2, vec3},
      prelude::*,
};
use avian2d::{math::*, parry::shape::SharedShape, prelude::*};

use crate::{resource::Score, states::*};
use crate::componet::*;
use crate::manager::*;
use crate::macros::*;



pub(super) fn plugin(app: &mut App) {

      app.observe(on_spawn_bricks);
      app.observe(on_despawn);
      app.observe(on_respawn);
    
}

#[derive(Event, Debug)]
pub struct Respawnbricks;

#[derive(Debug, Event)]
pub struct Despawnbricks;

#[derive(Event, Debug)]
pub struct Spawnbricks;


fn on_despawn(
      _trigger: Trigger<Despawnbricks>,
      mut cmd: Commands,
      mut bricks:Query<(Entity),(With<brick>,Without<player>)>,
      
  ) {
      for e in &bricks {
          cmd.entity(e).despawn_recursive();
      }
      cmd.trigger(Spawnbricks);
  }

fn on_respawn(
      _trigger: Trigger<Respawnbricks>,
      mut cmd: Commands,
) {
      cmd.trigger(Despawnbricks);
      //cmd.trigger(Despawn);
      //
}


fn spawn_sprite(commands: &mut Commands, asset_server: &AssetServer, position: Vec3, name: &str) {  
    commands.spawn((  
        SpriteBundle {  
            //texture: asset_server.load("branding/28_brick.png"),  
            transform: Transform::from_translation(position),  
            ..default()  
        },  
        RigidBody::Static,  
        multi!([(  
            "core",  
            anim_man!({  
                stable: {  
                    path: "branding/28_brick.png",  
                    size: (28, 8),  
                },  
                death: {  
                    path: "branding/28_brick_death.png",  
                    size: (28, 8),  
                    length: 4,  
                    fps: 16.0,  
                    next: "despawn",
                },
            })  
        )]),  
        Collider::rectangle(27., 7.),  
    ))  
    .insert(brick)  
    .insert(Name::from(name));  
}  


fn on_spawn_bricks(
      trigger: Trigger<Spawnbricks>,
      mut commands: Commands,
      asset_server: Res<AssetServer>
      //mut players:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity),(With<player>,Without<wall>,Without<brick>)>,
) {
     
    spawn_sprite(&mut commands, &asset_server, vec3(0., 76., 10.), "brick");  
    spawn_sprite(&mut commands, &asset_server, vec3(28., 76., 10.), "brick1");  
    spawn_sprite(&mut commands, &asset_server, vec3(56., 76., 10.), "brick3");  
    spawn_sprite(&mut commands, &asset_server, vec3(-56., 76., 10.), "brick4");  
    spawn_sprite(&mut commands, &asset_server, vec3(-28., 76., 10.), "brick5");  
    spawn_sprite(&mut commands, &asset_server, vec3(0., 68., 10.), "brick6");  
    spawn_sprite(&mut commands, &asset_server, vec3(-28., 68., 10.), "brick11");  
    spawn_sprite(&mut commands, &asset_server, vec3(28., 68., 10.), "brick7"); 

    spawn_sprite(&mut commands, &asset_server, vec3(0., 60., 10.), "brick8");  
    
    
    spawn_sprite(&mut commands, &asset_server, vec3(28., 60., 10.), "brick9");  
    spawn_sprite(&mut commands, &asset_server, vec3(-28., 60., 10.), "brick10");  
    
}
