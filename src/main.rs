use bevy::{
    asset::transformer, color::palettes::css::*, math::vec3, prelude::*, window::{PrimaryWindow, WindowResolution}
};
 
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};



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
    //.add_plugins(FlurxWryPlugin::default())
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, initcreate)
    .add_systems(Update, draw_example_collection)
    .insert_resource(ClearColor(Color::WHITE));

    app.run();
}

/// For spawning the main gameplay camera
#[derive(Bundle)]
struct MainCameraBundle {
    camera: Camera2dBundle,
    marker: MainCamera,
}

/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct MainCamera;

fn initcreate(
    mut commands: Commands,
    mut contexts: EguiContexts,
    asset_server: Res<AssetServer>,
    window: Query<Entity, With<PrimaryWindow>>,
){
    
    commands.spawn(Camera2dBundle::default());
    // Converts the `Window` attached the entity into a webview window. 
    /*
    commands.spawn((
        WryWebViewBundle {
            uri: WebviewUri::relative_local("1.html"),
            ..default()
        },
        AsChildBundle {
            parent: ParentWindow(window.single()),
            bounds: Bounds {
                position: Vec2::new(300., 100.),
                size: Vec2::new(500., 500.),
                min_size: Vec2::new(100., 100.),
            },
            ..default()
        },
    ));
    */
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32pink.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., 0., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., 32., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., -32., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., 64., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., -64., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., 96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-160., -96., 0.)),
        ..default()
    });
 
    //上边
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-128., 96., 0.)),
        ..default()

    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-96., 96., 0.)),
        ..default()

    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-64., 96., 0.)),
        ..default()

    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-32., 96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(0., 96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(32., 96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(64., 96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(96., 96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(128., 96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., 96., 0.)),
        ..default()
    });

    //下边
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-128., -96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-96., -96., 0.)),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-64., -96., 0.)),
        ..default()
    });
  
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(-32., -96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(0., -96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(32., -96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(64., -96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(96., -96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(128., -96., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., -96., 0.)),
        ..default()
    });



    //右边
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., 64., 0.)),
        ..default()
    });
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., 32., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., 0., 0.)),
        ..default()
    });
  
    //缺口
    /*
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., -32., 0.)),
        ..default()
    });
     */
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(160., -64., 0.)),
        ..default()
    });


    
    //延伸上
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(192., 0., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(224., 0., 0.)),
        ..default()
    });
    //延伸下
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(192., -64., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/32blank.png"),
        transform:Transform::from_translation(vec3(224., -64., 0.)),
        ..default()
    });



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
