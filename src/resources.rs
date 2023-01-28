use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
}
