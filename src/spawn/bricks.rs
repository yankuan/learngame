use bevy::{
      // color::palettes::css::*,
      math::{vec2, vec3},
      prelude::*,
};
use avian2d::{math::*, parry::shape::SharedShape, prelude::*};

use crate::{resource::Score, states::*};
use crate::componet::*;


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

fn on_spawn_bricks(
      trigger: Trigger<Spawnbricks>,
      mut commands: Commands,
      asset_server: Res<AssetServer>
      //mut players:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity),(With<player>,Without<wall>,Without<brick>)>,
) {
     
      commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(0., 76., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(28., 76., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick1"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(-56., 76., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick2"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(56., 76., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick3"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(-28., 76., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick4"));
    
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(0., 68., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick5"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(28., 68., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick6"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(-28., 68., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick7"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(0., 60., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick8"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(28., 60., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick9"));
    
        commands.spawn((SpriteBundle {
            texture: asset_server.load("branding/28_brick.png"),
            transform:Transform::from_translation(vec3(-28., 60., 10.)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(27.,7.), 
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
        ))
        .insert(brick).insert(Name::from("brick10"));
}
