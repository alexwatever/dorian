use bevy::prelude::*;

/// Set up the 3D camera and lighting
pub fn camera_setup(mut commands: Commands) {
    // Spawn 3D camera
    spawn_camera(&mut commands);

    // Add directional light
    spawn_directional_light(&mut commands);

    // Add ambient light
    spawn_ambient_light(&mut commands);
}

/// Spawn the 3D camera
fn spawn_camera(commands: &mut Commands) {
    // Create the camera
    let camera = Camera3d::default();
    let camera_transform = Transform::from_xyz(0.0, 3.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y);

    // Spawn the camera
    commands.spawn((camera, camera_transform));
}

/// Spawn the directional light
fn spawn_directional_light(commands: &mut Commands) {
    const ILLUMINANCE: f32 = 10000.0;
    const SHADOW_DEPTH_BIAS: f32 = 0.02;
    const SHADOW_NORMAL_BIAS: f32 = 1.8;

    // Create the directional light
    let directional_light = DirectionalLight {
        color: Color::WHITE,
        illuminance: ILLUMINANCE,
        shadows_enabled: true,
        shadow_depth_bias: SHADOW_DEPTH_BIAS,
        shadow_normal_bias: SHADOW_NORMAL_BIAS,
    };
    let directional_light_transform =
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -0.5));

    // Spawn the directional light
    commands.spawn((directional_light, directional_light_transform));
}

/// Spawn the ambient light
fn spawn_ambient_light(commands: &mut Commands) {
    const BRIGHTNESS: f32 = 75.0;

    // Create the ambient light
    let ambient_light = AmbientLight {
        color: Color::WHITE,
        brightness: BRIGHTNESS,
    };

    // Spawn the ambient light
    commands.insert_resource(ambient_light);
}
