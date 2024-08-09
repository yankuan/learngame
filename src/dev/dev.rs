use bevy::prelude::*;
use bevy_inspector_egui::quick::{StateInspectorPlugin,WorldInspectorPlugin};
use bevy_egui::{egui::{self, Label}, EguiContext, EguiContexts, EguiPlugin};
pub struct DebugPlugin;
use crate::states::*;
use crate::dev::style;

impl Plugin for DebugPlugin {
      fn build(&self, app: &mut bevy::prelude::App) {
          #[cfg(debug_assertions)]
          app.add_plugins(WorldInspectorPlugin::new())
              .add_plugins(StateInspectorPlugin::<Screen>::default())
              .add_plugins(StateInspectorPlugin::<GameState>::default());
          app.add_systems(Startup, debugset);

      }
}
fn debugset  (
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();
    style::set_style(ctx, style::Theme::light());
}

