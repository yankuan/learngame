use bevy::prelude::*;
use crate::states::GameState;


pub(super) fn plugin(app: &mut App) {
      app.add_sub_state::<GameState>();
      app.enable_state_scoped_entities::<GameState>();
}
