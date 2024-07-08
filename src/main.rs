use bevy::{
    prelude::*,
    window::WindowResolution,
    window::PrimaryWindow
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
    window: Query<Entity, With<PrimaryWindow>>,
){
    let camera = Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::AcesFitted,
        ..default()
    };
    commands.spawn(
        MainCameraBundle{
            camera,
            marker: MainCamera,
        }
    );
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

    let ctx = contexts.ctx_mut();
    style::set_style(ctx, style::Theme::light());


}