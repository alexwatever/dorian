use bevy::prelude::*;

/// Marker component for the player entity
#[derive(Component)]
pub struct Player;

/// Component for entities that can move
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Menu root marker (for cleanup)
#[derive(Component)]
pub struct MenuRoot;

/// Menu button kind
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum MenuButton {
    Start,
    Quit,
}

/// Current selection index for keyboard nav (0 = Start, 1 = Quit)
#[derive(Resource, Default)]
pub struct MenuSelection(pub usize);
