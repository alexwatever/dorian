use bevy::prelude::*;

// Modules
use crate::{
    components::player::{Player, Velocity},
    resources::GameTime,
};

// Player constants
const PLAYER_SIZE: f32 = 1.0;

/// Setup and spawn the player entity
pub fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Define the player cube
    let color = Color::srgb(0.25, 0.75, 0.25);

    // Create the cube mesh
    let mesh = meshes.add(Cuboid::new(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE).mesh());

    // Create the material
    let material = materials.add(StandardMaterial {
        base_color: color,
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });

    // Position the player
    let player_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    // Set the player velocity
    let player_velocity = Velocity::default();

    // Spawn the player
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        player_transform,
        Player,
        player_velocity,
    ));
}

/// Handle player movement based on keyboard input
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection), With<Camera3d>>,
    time: Res<Time>,
) {
    const MOVEMENT_SIZE: f32 = 1.0;
    const MOVEMENT_SPEED: f32 = 5.0;

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

    // Set the player bounds
    player_bounds(camera_query, player_transform);
}

/// Set the player bounds
fn player_bounds(
    camera_query: Query<(&Camera, &GlobalTransform, &Projection), With<Camera3d>>,
    mut player_transform: Mut<'_, Transform>,
) {
    if let Ok((_camera, camera_transform, projection)) = camera_query.get_single() {
        // Get the projection to extract FOV
        if let Projection::Perspective(perspective) = projection {
            // Calculate the perpendicular distance from camera to the player plane
            let camera_pos: Vec3 = camera_transform.translation();
            let camera_forward: Dir3 = camera_transform.forward();
            let distance_to_plane: f32 = camera_pos.z / camera_forward.z.abs();

            // Calculate visible world size at the player's Z position
            let half_height: f32 = distance_to_plane * (perspective.fov / 2.0).tan();
            let half_width: f32 = half_height * perspective.aspect_ratio;

            // Calculate the margin based on the player size
            let margin: f32 = PLAYER_SIZE / 2.0;

            // Set the player bounds
            player_transform.translation.x = player_transform
                .translation
                .x
                .clamp(-half_width + margin, half_width - margin);
            player_transform.translation.y = player_transform
                .translation
                .y
                .clamp(-half_height + margin, half_height - margin);
        }
    }
}

/// Set up the player animation
pub fn player_animate(
    mut player_materials: Query<&MeshMaterial3d<StandardMaterial>, With<Player>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_time: Res<GameTime>,
) {
    // Get the player material handle
    let material_handle: &MeshMaterial3d<StandardMaterial> = player_materials.single_mut();

    // Get the current game time
    let seconds: f32 = game_time.get();

    // Calculate the intensity of the colour based on the game time
    let intensity: f32 = (seconds.sin() + 1.0) / 2.0;

    // Animate the colour based on the game time
    let player_colour: Color = Color::srgb(
        0.25 + intensity * 0.25,
        0.75 - intensity * 0.25,
        0.25 + intensity * 0.5,
    );

    // Update the player colour
    if let Some(material) = materials.get_mut(&material_handle.0) {
        material.base_color = player_colour;
    }
}
