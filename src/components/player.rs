use bevy::prelude::*;

/// Marker component
#[derive(Component)]
pub struct Player;

/// Velocity component
#[derive(Component)]
pub struct Velocity {
    _x: f32,
    _y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { _x: 0.0, _y: 0.0 }
    }
}
