use bevy::prelude::*;

/// Game state resource (for future use)
#[derive(Resource, Default)]
pub struct GameState {
    pub score: u32,
    pub level: u32,
}

/// Game settings resource (for future use)
#[derive(Resource)]
pub struct GameSettings {
    pub master_volume: f32,
    pub difficulty: Difficulty,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            difficulty: Difficulty::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
} 