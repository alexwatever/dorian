# Dorian

## 🎮 About

Dorian is a small starter project A game development project built with [Bevy](https://bevyengine.org/). It's a work in progress.  

## 📁 Project Structure

The project follows Bevy's ECS architecture:  

```
dorian/
├── assets/ # Game assets
│ ├── sprites/ # Sprite images
│ ├── audio/ # Sound effects and music
│ └── fonts/ # Fonts
├── src/
│ ├── main.rs # App entry point (adds DefaultPlugins + GamePlugin)
│ ├── lib.rs # Defines GamePlugin and registers systems
│ ├── components.rs # ECS components (Player, Velocity, etc.)
│ ├── systems.rs # Systems (setup, movement, simple animation)
│ └── resources.rs # Future shared resources (game state/settings)
└── Cargo.toml # Dependencies and config
```

### Files at a glance

- `Cargo.toml`: Bevy 0.15, dev profile optimization  
- `src/main.rs`: Starts Bevy and adds plugins  
- `src/lib.rs`: `GamePlugin` wiring for startup/update systems  
- `src/components.rs`: `Player`, `Velocity`, etc.  
- `src/systems.rs`: camera + player setup, movement, simple color animation, etc.  
- `src/resources.rs`: scaffold for future shared state  

## 🚀 Run

Requirements:  
- Rust (stable), Cargo  

From the repo root:  
```bash
cargo run
```

## ⌨️ Controls

- Move: WASD or Arrow Keys
- Exit: Close the window or Ctrl+C in the terminal

## 🗺️ Roadmap

- Sprites and textures
- Basic collisions
- Obstacles / interactions
- Simple SFX/music
- Game states (menu/playing/game over)
