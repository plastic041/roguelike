use crate::components::{Position, Renderable};
use bevy::prelude::*;

/// Updates the position of all entities with a `Position` and `Renderable` component
pub fn update_positions(mut query: Query<(&Position, &mut Transform), With<Renderable>>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(position.x as f32, position.y as f32, 0.0);
    }
}
