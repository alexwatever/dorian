use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use systems::*;

/// App states
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

/// Main game plugin that sets up all game systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<components::MenuSelection>()
            // Camera available for both Menu and InGame
            .add_systems(Startup, setup_camera)
            // Menu
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(
                Update,
                (menu_keyboard, menu_mouse, menu_update_visuals).run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), cleanup_menu)
            // Game
            .add_systems(OnEnter(AppState::InGame), setup_player)
            .add_systems(
                Update,
                (player_movement, animate_player).run_if(in_state(AppState::InGame)),
            );
    }
}
