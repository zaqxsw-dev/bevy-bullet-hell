# Bevy Bullet Hell

A fast-paced bullet hell game built with the [Bevy engine][bevy]. Survive waves of enemies, collect experience, level up, and upgrade your abilities in this action-packed 2D shooter.

## Features

🎮 **Classic Bullet Hell Gameplay**
- Intense bullet-dodging action with waves of enemies
- Smooth 2D movement and shooting mechanics
- Dynamic particle effects and visual feedback

⚡ **Player Progression System**
- Experience points and leveling system
- Multiple skill upgrades including:
  - Attack speed and damage boosts
  - Movement speed improvements
  - Bullet penetration and ricochet effects
  - Health and defensive upgrades
  - Special abilities and spirit damage

🎯 **Combat Features**
- Auto-shooting system targeting enemies
- Dodge roll mechanic for evasive maneuvers
- Damage indicators and visual feedback
- Health and experience bars

🔧 **Technical Features**
- Built with Bevy 0.13.2
- Particle effects using bevy_hanabi
- Audio system with sound effects
- State management (Menu, Playing, Upgrade, Game Over)
- Responsive UI and HUD elements

## Installation

### Prerequisites

Make sure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs/).

### Building and Running

1. Clone the repository:
```bash
git clone https://github.com/zaqxsw-dev/bevy-bullet-hell.git
cd bevy-bullet-hell
```

2. Install system dependencies (Linux only):
```bash
# Ubuntu/Debian
sudo apt install libasound2-dev

# Arch Linux
sudo pacman -S alsa-lib

# Fedora
sudo dnf install alsa-lib-devel
```

3. Run the game:
```bash
cargo run --release
```

For development builds with faster compilation:
```bash
cargo run
```

## Controls

- **Movement**: WASD keys or Arrow keys
- **Shooting**: Automatic (aims toward mouse cursor)
- **Dodge Roll**: [Implementation may vary - check in-game]
- **Menu Navigation**: Mouse and keyboard

## How to Play

1. **Survive**: Avoid enemy bullets and attacks while staying alive
2. **Fight**: Your character automatically shoots at enemies near your mouse cursor
3. **Collect Experience**: Defeated enemies drop experience points
4. **Level Up**: Gain levels to unlock skill upgrade choices
5. **Upgrade**: Choose from various skills to improve your character
6. **Repeat**: Each wave gets progressively more challenging

## Game Mechanics

- **Health System**: Take damage from enemies and their attacks
- **Experience System**: Gain XP from defeating enemies to level up
- **Skill Tree**: Multiple upgrade paths for different playstyles
- **Enemy Waves**: Continuous spawning with increasing difficulty
- **Particle Effects**: Visual feedback for hits, explosions, and abilities

## Building for Web

To build for web deployment:

1. Install trunk:
```bash
cargo install --locked trunk
```

2. Add the wasm target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Serve locally:
```bash
trunk serve
```

This will serve the game on `http://localhost:8080` with automatic reloading.

## Development

The game is structured using Bevy's plugin system:
- **Player Plugin**: Handles player movement, shooting, and stats
- **Enemy Spawn Plugin**: Manages enemy spawning and AI
- **UI Plugins**: Health bars, experience bars, damage indicators
- **Audio Plugin**: Sound effects and music
- **Game State Management**: Menu, gameplay, upgrades, game over

## Credits

**Author**: Bogdan Lipovtsev (megafreelancer2012@gmail.com)

**Built with**:
- [Bevy Engine][bevy] - Game engine
- [bevy_kira_audio] - Audio system  
- [bevy_hanabi] - Particle effects
- [bevy_asset_loader] - Asset management

## License

This project is licensed under [CC0 1.0 Universal](LICENSE) except some content of `assets` and the Bevy icons in the `build` directory (see [Credits](credits/CREDITS.md)).

[bevy]: https://bevyengine.org/
[bevy_kira_audio]: https://github.com/NiklasEi/bevy_kira_audio
[bevy_hanabi]: https://github.com/djeedai/bevy_hanabi
[bevy_asset_loader]: https://github.com/NiklasEi/bevy_asset_loader
