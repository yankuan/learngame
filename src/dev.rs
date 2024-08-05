use bevy::prelude::*;
use bevy_inspector_egui::quick::{StateInspectorPlugin,WorldInspectorPlugin};

pub struct DebugPlugin;
use crate::states::*;

impl Plugin for DebugPlugin {
      fn build(&self, app: &mut bevy::prelude::App) {
          #[cfg(debug_assertions)]
          app.add_plugins(WorldInspectorPlugin::new())
              .add_plugins(StateInspectorPlugin::<Screen>::default())
              .add_plugins(StateInspectorPlugin::<GameState>::default());

      }
}

