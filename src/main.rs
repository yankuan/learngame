use bevy::{
    asset::transformer, color::palettes::css::*, core_pipeline::core_2d::graph::input, ecs::label, input::keyboard, log::tracing_subscriber::fmt::writer::BoxMakeWriter, math::{vec2, vec3}, prelude::*, window::{PrimaryWindow, WindowResolution}
};
 
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy_egui::{egui::{self, Label}, EguiContext, EguiContexts, EguiPlugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use avian2d::{math::*, parry::shape::SharedShape, prelude::*};


mod style;


fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                //设置窗口大小 1100*750
                primary_window: Some(Window {
                    #[cfg(target_os = "windows")]
                    position: WindowPosition::Centered(MonitorSelection::Primary), //窗口居中
                    resolution: WindowResolution::new(1200.0, 800.0),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(LogDiagnosticsPlugin::default())
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(PhysicsPlugins ::default().with_length_unit(0.0))
    .add_plugins(PhysicsDebugPlugin::default())
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, (initcreate,initcreate2))
    .add_systems(Update, (draw_example_collection,player_about))
    .insert_resource(ClearColor(Color::WHITE));

    app.run();
}


/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct player;

#[derive(Component)]
pub struct ball;
/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct wall;

/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct brick;

fn initcreate2(   
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(0., 76., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(28., 76., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick1"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(-56., 76., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick2"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(56., 76., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick3"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(-28., 76., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick4"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(0., 68., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick5"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(28., 68., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick6"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(-28., 68., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick7"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(0., 60., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick8"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(28., 60., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick9"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/28_brick.png"),
        transform:Transform::from_translation(vec3(-28., 60., 0.)),
        ..default()
    },
    RigidBody::Static,
    Collider::rectangle(27.,7.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(brick).insert(Name::from("brick10"));


}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect, States, Hash)]
pub enum Ballstatus {
    #[default]
    Nomove,
    Move
}



fn initcreate(
    mut commands: Commands,
    mut contexts: EguiContexts,
    asset_server: Res<AssetServer>,
    window: Query<Entity, With<PrimaryWindow>>,
){
    
    let mut projection1 = OrthographicProjection::default();
    projection1.scale = 0.28;


    commands.spawn(Camera2dBundle{
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.,0., 9.)),
        projection:projection1,
        ..default()
    });
    
    //commands.spawn(Camera2dBundle::default());
    //ball
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/8ball.png"),
            transform: Transform::from_translation(Vec3::new(0.,-63., 1.)),
            ..default()
        },
        ball,
        Ballstatus::Nomove,
        RigidBody::Dynamic,
        GravityScale(20.0),
        Collider::circle(4.),
        //Collider::rectangle(32.,32.), 
        SweptCcd::default(),
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 0.0))
        ))
        .insert(Name::from("ball")
    );


    //player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/10_64player.png"),
            transform: Transform::from_translation(Vec3::new(0.,-74., 1.)),
            ..default()
        },
        player,
        RigidBody::Kinematic,
        GravityScale(0.0),
        //Collider::circle(16.),
        Collider::rectangle(64.,11.), 
        //SweptCcd::default(),
        //DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 0.0))
        ))
        .insert(Name::from("player")
    );


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., 0., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., 32., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));



    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., -32., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., 64., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    

    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., -64., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
 
    //上边
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-128., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-96., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-64., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-32., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(0., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(32., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(64., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(96., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(128., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    
    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., 96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    //下边
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-128., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-96., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-64., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
  
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-32., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(0., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(32., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(64., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(96., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(128., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., -96., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));



    //右边
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., 64., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., 32., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., 0., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

  
    //缺口
    /*
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., -32., 0.)),
        ..default()
    });
     */
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., -64., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));



    
    //延伸上
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(192., 0., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(224., 0., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    //延伸下
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(192., -64., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(224., -64., 0.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    //自定义碰撞
    commands.spawn((
        RigidBody::Static, 
        Collider::rectangle(286.0, 32.),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 96.0, 0.0)),
    ));
    
    commands.spawn((
        RigidBody::Static, 
        Collider::rectangle(286.0, 32.),
        TransformBundle::from_transform(Transform::from_xyz(0.0, -96.0, 0.0)),
    ));
    commands.spawn((
        RigidBody::Static, 
        Collider::rectangle(32.0, 224.),
        TransformBundle::from_transform(Transform::from_xyz(160.0, 0.0, 0.0)),
    ));
    commands.spawn((
        RigidBody::Static, 
        Collider::rectangle(32.0, 224.),
        TransformBundle::from_transform(Transform::from_xyz(-160.0, 0.0, 0.0)),
    ));

    let ctx = contexts.ctx_mut();
    style::set_style(ctx, style::Theme::light());

}


fn draw_example_collection(
    mut gizmos: Gizmos,
    mouse:Res<ButtonInput<MouseButton>>,
    game_camera_query: Query<(&Camera, &GlobalTransform)>,
    mut windows: Query<&mut Window>,
) {
    /* 
    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.0,
            UVec2::new(20, 20),
            Vec2::new(32., 32.),
            // Dark gray
            LinearRgba::gray(0.00),
        )
        .outer_edges();
        */

        if mouse.just_pressed(MouseButton::Left) {
            let mut window = windows.single_mut();
            let (camera, camera_transform) = game_camera_query.single();
            if let Some(position) = window.cursor_position() {
                println!("鼠标点击点的坐标：(x={}, y={})", position.x, position.y);
               
                //trans.translation.y = position.x; //214 反 50
                //trans.translation.x = position.y //212 正
                // Calculate a world position based on the cursor's position.
                let Some(point) = camera.viewport_to_world_2d(camera_transform, position) else {
                    return;
                };
                println!("世界坐标：(x={}, y={})", point.x, point.y);
    
            }
        }
        
 
}


fn player_about(
    mut commands: Commands,
    keyboard:Res<ButtonInput<KeyCode>>,
    mut players:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity),(With<player>,Without<wall>,Without<brick>)>,
    mut balls:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity,&mut Ballstatus),(With<ball>,Without<player>,Without<wall>,Without<brick>)>,
    mut bricks:Query<(Entity),(With<brick>,Without<player>)>,
    mut collision_event_reader: EventReader<Collision>,
    mut collision_event_reader_start: EventReader<CollisionStarted>,
    delta: Res<Time>,
){

    let Ok((ent_ball,mut trans_ball, mut linear_ball,mut ang_ball, mut ballsta)) = balls.get_single_mut() else { return;};
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
    
    if keyboard.pressed(KeyCode::ArrowUp) {
        //linear.y = 1. * 5000. * delta.delta_seconds();
        //trans.translation.y += 1. * 100. * delta.delta_seconds();
        if *ballsta == Ballstatus::Nomove {
            linear_ball.y = 350.;
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
    
     
    //判断失败 不以碰撞检测为标志，以ball的Y轴与-96的关系判断
    //println!("{}",trans_ball.translation.y);
}    