# Dorian

## 🎮 About

Dorian is a small game development project built with [Bevy](https://bevyengine.org/). It's a work in progress.  

## 📁 Project Structure

The project follows Bevy's ECS architecture:  

```
dorian/
├── assets/ # Game assets
│ ├── audio/ # Sound effects and music
│ ├── fonts/ # Fonts
│ └── sprites/ # Sprite images
├── src/
│ ├── main.rs # App entry point (adds DefaultPlugins + GamePlugin)
│ ├── lib.rs # Defines and registers the game plugin and its systems
│ ├── error.rs # Error types
│ ├── resources.rs # Future shared resources (game state/settings)
│ ├── components/ # ECS components
│ │ ├── button.rs # Button component
│ │ ├── menu.rs # Menu component
│ │ └── player.rs # Player component
│ └── systems/ # Systems
│   ├── camera.rs # Camera system
│   ├── menu.rs # Menu system
│   ├── player.rs # Player system
│   └── time.rs # Time system
└── Cargo.toml # Dependencies and config
```

### Files at a glance

- `Cargo.toml`: Bevy 0.15, dev profile optimization  
- `src/main.rs`: Starts Bevy and adds plugins  
- `src/lib.rs`: Setup for the game plugin and its systems  
- `src/resources.rs`: Scaffold for shared state  
- `src/components/`: Player, Menu, etc.  
- `src/systems/`: Camera, player setup, movement, simple color animation, etc.  

## 🚀 Run

Requirements:  
- Rust (stable), Cargo  

From the repo root:  
```bash
cargo run
```

## ⌨️ Controls

- Move: WASD or Arrow Keys
- Exit: ESC to access the in-game menu, close the window, or Ctrl+C in the terminal

## 🗺️ Roadmap (WIP)

- Game states (menu/playing/game over)
- Basic collisions
- Obstacles / interactions
- Simple SFX/music
- Sprites and textures
