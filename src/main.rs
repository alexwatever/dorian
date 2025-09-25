#![allow(unused_crate_dependencies)]
use bevy::prelude::*;
use dorian::GamePlugin;

fn main() {
    App::new()
        // Add Bevy's default plugins (windowing, rendering, input, etc.)
        .add_plugins(DefaultPlugins)
        // Add our game plugin
        .add_plugins(GamePlugin)
        // Run the app
        .run();
}
