use bevy::{
    asset::transformer, color::palettes::css::*, core_pipeline::core_2d::graph::input, ecs::label, input::keyboard, log::tracing_subscriber::fmt::writer::BoxMakeWriter, math::{vec2, vec3}, prelude::*, window::{PrimaryWindow, WindowResolution}
};
 
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::core_pipeline::tonemapping::Tonemapping;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use avian2d::{math::*, parry::shape::SharedShape, prelude::*};
use bevy_light_2d::prelude::*;
use bevy::{sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use states::GameState;

mod states;
mod screen; //屏幕显示
mod game; //游戏
mod death_state;
mod victory;
mod assets;
mod ui;
mod componet;
mod resource;
mod spawn;
mod background;
mod macros;
mod manager;
mod consts;
mod math;
mod mat;
mod game_time;
mod dev;


use crate::componet::*;
use crate::resource::Score;
use crate::background::BackgroundKind;
use crate::dev::dev::*;
use crate::manager::*;


use crate::macros::*;
use crate::{
    assets::{FontKey, HandleMap}
};



const SCOREBOARD_FONT_SIZE: f32 = 50.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::WHITE;
//const SCORE_COLOR: Color= Color::srgb(1.0, 0.5, 0.5);
const SCORE_COLOR: Color = Color::WHITE;



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
    //.add_plugins(LogDiagnosticsPlugin::default())
    //.add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(PhysicsPlugins ::default().with_length_unit(0.0))
    //.add_plugins(PhysicsDebugPlugin::default())
    .add_plugins(Light2dPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(screen::plugin)
    .add_plugins(game::plugin)
    .add_plugins(death_state::plugin)
    .add_plugins(victory::plugin)
    .add_plugins(assets::plugin)
    .add_plugins(ui::plugin)
    .add_plugins(spawn::plugin)
    .add_plugins(game_time::plugin)
    .add_systems(Startup, (initcreate,initcreate2,initcreate3))
    .add_systems(Update, (draw_example_collection,update_scoreboard))
    .insert_resource(Score(0))
    .insert_resource(ClearColor(Color::srgb(0.678, 0.847, 0.902)));
    //.insert_resource(ClearColor(Color::BLACK));

    app.run();
}


fn initcreate3(
    mut commands: Commands,
){

    //BackgroundKind::SkyOnly.spawn(default(), &mut commands);
    BackgroundKind::Zenith.spawn(default(), &mut commands);
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

fn initcreate2(   
    mut commands: Commands,
    font_handles: Res<HandleMap<FontKey>>,
    asset_server: Res<AssetServer>
){
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
    
    // Scoreboard
    commands.spawn((
        ScoreboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(43.),
            left: Val::Px(500.),
            ..default()
        }),
    ));
    //计时器
    commands.spawn((
        GameTimeText,
        TextBundle {
            text: Text::from_section(
                "0:00.00",
                TextStyle {
                    font: font_handles[&FontKey::GeoFont].clone_weak(),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: Color::from(WHITE_SMOKE),
                },
            ),
            style: Style {
                width: Val::Px(10.),
                position_type: PositionType::Absolute,
                top: Val::Px(730.),
                left: Val::Px(500.),
                ..default()
            },
            ..default()
        },
    ));


}

fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.to_string();
}


fn initcreate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        transform: Transform::from_translation(Vec3::new(0.,0., 99.)),
        projection:projection1,
        ..default()
    });

    let mesh_handle = Mesh2dHandle(meshes.add(Circle::new(10.)));
    

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: materials.add(Color::srgba_u8(190, 107, 107, 255)),
        transform: Transform::from_translation(Vec3::new(100.,50., 1.)),
        ..default()
    }).insert(Name::from("sun"));
    

    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color:Color::srgba_u8(190, 107, 107, 255),
            radius: 500.0,
            intensity: 1.2,
            falloff:180.,
            //cast_shadows:true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(100.,50., 1.)),
        ..default()
    }).insert(Name::from("sunlight"));
    
    //commands.spawn(Camera2dBundle::default());
    //ball
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/32ball.png"),
            //transform: Transform::from_translation(Vec3::new(0.,-64., 10.)),
            transform:Transform { 
                translation: Vec3::new(0.,-64., 10.),
                scale:Vec3::new(0.25,0.25, 1.),
                ..default()
            },
            ..default()
        },
        ball,
        Ballstatus::Nomove,
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::circle(16.),
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
            transform: Transform::from_translation(Vec3::new(0.,-74., 10.)),
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
        texture: asset_server.load("branding/32base.png"),
        transform:Transform::from_translation(vec3(-160., 0., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32base.png"),
        transform:Transform::from_translation(vec3(-160., 32., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));



    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32base.png"),
        transform:Transform::from_translation(vec3(-160., -32., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32base.png"),
        transform:Transform::from_translation(vec3(-160., 64., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    

    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32base.png"),
        transform:Transform::from_translation(vec3(-160., -64., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top.png"),
        transform:Transform::from_translation(vec3(-160., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
 
    //上边
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top.png"),
        transform:Transform::from_translation(vec3(-128., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top.png"),
        transform:Transform::from_translation(vec3(-96., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top.png"),
        transform:Transform::from_translation(vec3(-64., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top1.png"),
        transform:Transform::from_translation(vec3(-32., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top2.png"),
        transform:Transform::from_translation(vec3(0., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top3.png"),
        transform:Transform::from_translation(vec3(32., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top.png"),
        transform:Transform::from_translation(vec3(64., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top.png"),
        transform:Transform::from_translation(vec3(96., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top.png"),
        transform:Transform::from_translation(vec3(128., 96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    
    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32top.png"),
        transform:Transform::from_translation(vec3(160., 96., 10.)),
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
        transform:Transform::from_translation(vec3(-128., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-96., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-64., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
  
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-32., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(0., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(32., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(64., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(96., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));
    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(128., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));


    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., -96., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));



    //右边
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32right.png"),
        transform:Transform::from_translation(vec3(160., 64., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32right.png"),
        transform:Transform::from_translation(vec3(160., 32., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32right.png"),
        transform:Transform::from_translation(vec3(160., 0., 10.)),
        ..default()
    },
    //RigidBody::Static,
    //Collider::rectangle(32.,32.), 
    //DebugRender::default().with_collider_color(Color::srgb(1.0, 1.0, 0.0))
    ))
    .insert(wall).insert(Name::from("wall"));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32right.png"),
        transform:Transform::from_translation(vec3(160., -32., 10.)),
        ..default()
    });
    commands.spawn((SpriteBundle {
        texture: asset_server.load("branding/32right.png"),
        transform:Transform::from_translation(vec3(160., -64., 10.)),
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
        transform:Transform::from_translation(vec3(192., 0., 10.)),
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
    )).insert(checkover).insert(Name::from("checkover"));


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


