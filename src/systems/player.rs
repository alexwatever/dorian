use bevy::prelude::*;

// Modules
use crate::{
    components::player::{Player, Velocity},
    resources::GameTime,
};

/// Setup and spawn the player entity
pub fn player_setup(mut commands: Commands) {
    // Define the player sprite
    let color = Color::srgb(0.25, 0.75, 0.25);
    let size = Vec2::new(50.0, 50.0);

    // Create the player sprite
    let player_sprite = Sprite {
        color,
        custom_size: Some(size),
        ..default()
    };

    // Position the player
    let player_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    // Set the player velocity
    let player_velocity = Velocity::default();

    // Spawn the player
    commands.spawn((player_sprite, player_transform, Player, player_velocity));
}

/// Handle player movement based on keyboard input
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    const MOVEMENT_SIZE: f32 = 1.0;
    const MOVEMENT_SPEED: f32 = 200.0;

    // Get the player transform
    let mut player_transform = player_transform.single_mut();

    // Init the direction
    let mut direction = Vec3::ZERO;

    // Set the direction based on the keyboard input
    match keyboard_input.get_pressed().next() {
        // Move up
        Some(KeyCode::KeyW) | Some(KeyCode::ArrowUp) => direction.y += MOVEMENT_SIZE,
        // Move down
        Some(KeyCode::KeyS) | Some(KeyCode::ArrowDown) => direction.y -= MOVEMENT_SIZE,
        // Move left
        Some(KeyCode::KeyA) | Some(KeyCode::ArrowLeft) => direction.x -= MOVEMENT_SIZE,
        // Move right
        Some(KeyCode::KeyD) | Some(KeyCode::ArrowRight) => direction.x += MOVEMENT_SIZE,
        // Unsupported movement
        _ => {
            debug!(
                "Unhandled keyboard input: {:?}",
                keyboard_input.get_pressed().next()
            );
        }
    }

    // Normalize the direction to prevent faster movement diagonally
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    // Move the player
    player_transform.translation += direction * MOVEMENT_SPEED * time.delta_secs();
}

/// Set up the player animation
pub fn player_animate(
    mut player_sprite: Query<&mut Sprite, With<Player>>,
    game_time: Res<GameTime>,
) {
    // Get the player sprite
    let mut player_sprite = player_sprite.single_mut();

    // Get the current game time
    let seconds = game_time.get();

    // Calculate the intensity of the colour based on the game time
    let intensity = (seconds.sin() + 1.0) / 2.0;

    // Animate the colour based on the game time
    player_sprite.color = Color::srgb(
        0.25 + intensity * 0.25,
        0.75 - intensity * 0.25,
        0.25 + intensity * 0.5,
    );
}
