use bevy::prelude::*;

#[derive(Component)]
pub struct Renderable;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Player;
