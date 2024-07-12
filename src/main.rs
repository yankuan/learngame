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
    .add_systems(Startup, initcreate)
    .add_systems(Update, (draw_example_collection,player_about))
    .insert_resource(ClearColor(Color::WHITE));

    app.run();
}


/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct player;

/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct wall;

fn initcreate(
    mut commands: Commands,
    mut contexts: EguiContexts,
    asset_server: Res<AssetServer>,
    window: Query<Entity, With<PrimaryWindow>>,
){
    
    let mut projection1 = OrthographicProjection::default();
    projection1.scale = 0.3;


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

    //player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/32pink.png"),
            ..default()
        },
        player,
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::circle(16.),
        //Collider::rectangle(32.,32.), 
        SweptCcd::default(),
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
        Collider::rectangle(320.0, 32.),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 96.0, 0.0)),
    ));
    commands.spawn((
        RigidBody::Static, 
        Collider::rectangle(320.0, 32.),
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
    mut players:Query<(&mut LinearVelocity,&mut AngularVelocity),(With<player>,Without<wall>)>,
    mut walls:Query<(&mut Transform),(With<wall>,Without<player>)>,
    mut collision_event_reader: EventReader<Collision>,
    delta: Res<Time>,
){
    
    //let (mut trans,mut linear) =  players.get_single_mut().expect("没有获取player实体");
    for (mut linear,mut ang) in &mut players {
        if keyboard.just_pressed(KeyCode::ArrowLeft) {
            linear.x = -1. * 5000. * delta.delta_seconds();
        }
        if keyboard.just_pressed(KeyCode::ArrowRight) {
            linear.x = 1. * 5000. * delta.delta_seconds();
        }
        if keyboard.just_pressed(KeyCode::ArrowUp) {
            linear.y = 1. * 5000. * delta.delta_seconds();
        }
        if keyboard.just_pressed(KeyCode::ArrowDown) {
            linear.y = -1. * 5000. * delta.delta_seconds();
        } 
        for Collision(contacts) in collision_event_reader.read() {

            if contacts.entity1.index() == 9  || contacts.entity1.index() == 9 {
                
                linear.y = 0.;
                linear.x = 0.;
                ang.0 = 0.;
            }
        }
    }
}    