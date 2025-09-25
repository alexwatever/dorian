use bevy::prelude::*;

// Modules
use crate::resources::GameTime;

/// Tick the game time
pub fn tick_game_time(mut game_time: ResMut<GameTime>, time: Res<Time>) {
    game_time.0 += time.delta_secs();
}
