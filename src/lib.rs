use bevy::prelude::*;

// Modules
mod components;
mod error;
mod resources;
mod systems;
use crate::{
    components::menu::{IngameMenu, Menu, MenuSelection, StartMenu},
    resources::{AppState, PauseState},
    systems::{
        camera::camera_setup,
        menu::pause_toggle,
        player::{player_animate, player_movement, player_setup},
        time::tick_game_time,
    },
};

/// Main game plugin that sets up all game systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Setup the time resource
        app.init_resource::<resources::GameTime>().add_systems(
            Update,
            tick_game_time
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(PauseState::Running)),
        );

        // Setup the application state
        app.init_state::<AppState>()
            // Setup the pause state
            .init_state::<PauseState>()
            // Setup the menu selection resource
            .init_resource::<MenuSelection>()
            // Setup the camera
            .add_systems(Startup, camera_setup)
            // Setup the start menu
            .add_systems(OnEnter(AppState::Menu), StartMenu::setup)
            .add_systems(
                Update,
                (
                    StartMenu::keyboard_input,
                    StartMenu::mouse_input,
                    StartMenu::update_visuals,
                )
                    .run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), StartMenu::cleanup)
            // Setup the in-game menu
            .add_systems(OnEnter(PauseState::Paused), IngameMenu::setup)
            .add_systems(
                Update,
                (
                    IngameMenu::keyboard_input,
                    IngameMenu::mouse_input,
                    IngameMenu::update_visuals,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(PauseState::Paused)),
            )
            .add_systems(OnExit(PauseState::Paused), IngameMenu::cleanup)
            // Setup the in-game pause toggle
            .add_systems(Update, pause_toggle.run_if(in_state(AppState::InGame)))
            // Setup the player
            .add_systems(OnEnter(AppState::InGame), player_setup)
            .add_systems(
                Update,
                (player_movement, player_animate)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}
