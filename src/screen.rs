use bevy::prelude::*;
use crate::states::Screen;


pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();
}
