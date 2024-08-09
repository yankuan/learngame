use bevy::prelude::*;

/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct player;

#[derive(Component)]
pub struct ball;
/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct wall;

#[derive(Component)]
pub struct checkover;

/// Marker component for the main gameplay camera
#[derive(Component)]
pub struct brick;

#[derive(Component)]
pub struct ScoreboardUi;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect, States, Hash)]
pub enum Ballstatus {
    #[default]
    Nomove,
    Move
}

#[derive(Debug, Component)]
pub struct GameTimeText;


