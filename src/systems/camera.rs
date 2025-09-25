use bevy::prelude::*;

/// Set up the 2D camera
pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
