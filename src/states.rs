use bevy::prelude::*;

/// The game's main screen states.
#[derive(Clone, Eq, PartialEq, Debug, Copy, Hash, Default, States, Reflect)]
pub enum Screen {
    #[default]
    Playing,
    Loading,
    Title,
    // Credits,
    Splash,
    Restart,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Reflect)]
#[source(Screen = Screen::Playing)]
pub enum GameState {
    #[default]
    Wait,
    Playing,
    Victory,
    Death,
    Pause,
}

