use bevy::prelude::*;

/// App state
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

/// Pause state
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}

/// Game state
#[derive(Resource, Default)]
pub struct GameState {
    _score: u32,
    _level: u32,
}

/// Game time
#[derive(Resource, Default)]
pub struct GameTime(pub f32);

impl GameTime {
    /// Getter for the current game time
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// Game settings
#[derive(Resource)]
pub struct GameSettings {
    _master_volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            _master_volume: 1.0,
        }
    }
}
