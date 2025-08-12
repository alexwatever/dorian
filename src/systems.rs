use crate::components::*;
use crate::AppState;
use bevy::prelude::*;

/// Set up the 2D camera (spawned when entering InGame)
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Spawn the player entity (spawned when entering InGame)
pub fn setup_player(mut commands: Commands) {
    // For now, we'll use a simple colored square as the player
    commands.spawn((
        Sprite {
            color: Color::srgb(0.25, 0.75, 0.25),     // Green color
            custom_size: Some(Vec2::new(50.0, 50.0)), // 50x50 pixels
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
        Velocity::default(),
    ));
}

/// Handle player movement based on keyboard input
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let speed = 200.0; // pixels per second

    let mut direction = Vec3::ZERO;

    // Handle WASD and arrow keys
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    // Normalize diagonal movement
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    // Apply movement
    player_transform.translation += direction * speed * time.delta_secs();
}

/// Simple animation system (changes color over time as an example)
pub fn animate_player(mut query: Query<&mut Sprite, With<Player>>, time: Res<Time>) {
    let mut sprite = query.single_mut();

    // Simple color animation based on time
    let seconds = time.elapsed_secs();
    let intensity = (seconds.sin() + 1.0) / 2.0; // Value between 0 and 1

    sprite.color = Color::srgb(
        0.25 + intensity * 0.25,
        0.75 - intensity * 0.25,
        0.25 + intensity * 0.5,
    );
}

// ---------- Menu ----------

pub fn setup_menu(mut commands: Commands, mut selection: ResMut<MenuSelection>) {
    selection.0 = 0;

    let root = commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .id();

    let button_node = || Node {
        width: Val::Px(220.0),
        height: Val::Px(44.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(8.0)),
        ..default()
    };

    // Start button
    commands.entity(root).with_children(|parent| {
        parent
            .spawn((
                Button,
                MenuButton::Start,
                button_node(),
                BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
            ))
            .with_children(|p| {
                p.spawn((
                    Text::new("Start"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                ));
            });
    });

    // Quit button
    commands.entity(root).with_children(|parent| {
        parent
            .spawn((
                Button,
                MenuButton::Quit,
                button_node(),
                BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
            ))
            .with_children(|p| {
                p.spawn((
                    Text::new("Quit"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                ));
            });
    });
}

pub fn cleanup_menu(mut commands: Commands, roots: Query<Entity, With<MenuRoot>>) {
    for e in &roots {
        commands.entity(e).despawn_recursive();
    }
}

pub fn menu_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<MenuSelection>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    // Navigate
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        selection.0 = selection.0.saturating_sub(1);
    }
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        selection.0 = (selection.0 + 1).min(1);
    }

    // Activate
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        match selection.0 {
            0 => next_state.set(AppState::InGame),
            1 => {
                let _ = exit.send(AppExit::Success);
            }
            _ => {}
        }
    }
}

pub fn menu_mouse(
    mut interactions: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut selection: ResMut<MenuSelection>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in &mut interactions {
        match *interaction {
            Interaction::Pressed => match button {
                MenuButton::Start => next_state.set(AppState::InGame),
                MenuButton::Quit => {
                    let _ = exit.send(AppExit::Success);
                }
            },
            Interaction::Hovered => {
                selection.0 = match button {
                    MenuButton::Start => 0,
                    MenuButton::Quit => 1,
                };
            }
            Interaction::None => {}
        }
    }
}

pub fn menu_update_visuals(
    selection: Res<MenuSelection>,
    mut q: Query<(&MenuButton, &mut BackgroundColor)>,
) {
    for (button, mut bg) in &mut q {
        let selected = matches!(
            (selection.0, button),
            (0, MenuButton::Start) | (1, MenuButton::Quit)
        );
        bg.0 = if selected {
            Color::srgb(0.30, 0.30, 0.45)
        } else {
            Color::srgb(0.15, 0.15, 0.20)
        };
    }
}
