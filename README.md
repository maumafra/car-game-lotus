![white-lancer-icon](https://github.com/user-attachments/assets/f1e2a82b-b466-4c98-b919-77166ca0dc88)
# CyberLancer: Neon Rush
--------------

**CyberLancer** is a cyberpunk racing game built in **Rust** using the **Lotus Engine**. Inspired by retro classics like *Road Fighter (NES)* and *Out Run*, you pilot a **Cyber-Lancer** through a neon-drenched bridge, dodging obstacles and surviving as long as possible in a dystopian world.

## 🌆 Overview
- **Genre**: Arcade Racing / Survival
- **Aesthetic**: Retro-futuristic (pixel art + cyberpunk neon)
- **Engine**: [Lotus](https://github.com/zenialexandre/lotus) (Rust)
- **Status**: Playable Prototype

## 🚗 Features  
- **Arcade Controls**: Tight handling with instant response.
- **Procedural Generation**: Dynamic obstacles.
- **Scoring System**: Survive longer = higher score.

![cyberlancer](https://github.com/user-attachments/assets/ef3869d5-a7e5-4d89-bb94-c9dd56bfeb05)

## 🎮 Controls
| Key               | Action                          |
|-------------------|---------------------------------|
| <kbd>A</kbd>      | Steer **left**                  |
| <kbd>D</kbd>      | Steer **right**                 |
| <kbd>W</kbd>      | **Navigates** through menus     |
| <kbd>S</kbd>      | **Navigates** through menus     |
| <kbd>X</kbd>      | **Horn**                        |
| <kbd>Esc</kbd>    | **Pause**/Unpause game          |
| <kbd>Enter</kbd>  | **Interact**                    |
| <kbd>Space</kbd>  | **Slowmo** (WIP)                |

## 🏆 Current Progress
| Key               | Action                          |
|-------------------|---------------------------------|
| Cars Generation   | ✅ Complete                    |
| Collisions        | ✅ Complete                    |
| Main Menu         | ✅ Complete                    |
| Pause             | ✅ Complete                    |
| Game Over         | ✅ Complete                    |
| Music/Sounds      | 🔧 Fine-tuning                 |
| Art               | 🔧 Fine-tuning                 |
| Power Ups         | 🚧 In Development              |
| Save Files         | 🚧 In Development              |
| Enemy AI          | ❌ Planned                     |
| In-game Achievements| ❌ Planned                   |
| Track Generation  | ❌ Planned                     |
| Multiple Tracks/Cars| ❌ Planned                   |

## 📂 Project Structure  
This project adheres to the Entity Component System (ECS) architecture, organizing code around entities that aggregate related components, resources, and systems:
```shell
car-game-lotus/
├── assets/
│ ├── fonts/
│ ├── sounds/
│ ├── sprites/
│ └── ...
├── src/
│ ├── cars/
│ │ ├── mod.rs
│ │ ├── components.rs
│ │ ├── resources.rs
│ │ └── systems.rs
│ ├── player/
│ ├── common/
│ ├── ...
│ └── main.rs
└── Cargo.toml
```

## 🛠️ Tech Stack  
```rust
[dependencies]
lotus_engine = "0.1.24"  // Game engine
rand = "0.9.0"          // Procedural generation
```

## 🤝 Contributing
1. Fork the project.
2. Create a branch:
   ```bash
   git switch -c feat/new-mechanic
   ```
3. Submit a pull request.
