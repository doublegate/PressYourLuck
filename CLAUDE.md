# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Press Your Luck is an authentic recreation of the 1983-1986 CBS game show, built in Rust using the ggez game framework (v0.9). The game features a hybrid audio system (file-based with procedural fallback), a complete animation system with particle effects, vector-based graphics with Canvas/Mesh rendering, and implements the complete game rules including question rounds, the Big Board, passing mechanics, and 4-Whammy elimination.

**Current Version**: 2.1.0

## Build Commands

```bash
# Development build
cargo build
cargo run

# Release build (optimized)
cargo build --release
./target/release/press-your-luck

# Run tests
cargo test

# Format check
cargo fmt --check

# Lint
cargo clippy -- -D warnings
```

### Linux Prerequisites

Requires ALSA and udev development libraries:
```bash
# Ubuntu/Debian
sudo apt install libasound2-dev libudev-dev

# Fedora
sudo dnf install alsa-lib-devel libudev-devel

# Arch/CachyOS
sudo pacman -S alsa-lib systemd-libs
```

## Architecture

The codebase follows a modular architecture with clear separation between game logic, rendering, audio, and UI:

```
src/
├── main.rs              # Entry point, ggez EventHandler, game loop integration
├── game/mod.rs          # Game state, rules, all game logic
├── audio/mod.rs         # Hybrid audio engine (file-based + procedural fallback)
├── animation/           # Phase 2: Animation System
│   ├── mod.rs           # Module exports
│   ├── atlas.rs         # Sprite atlas management (grid/packed)
│   ├── effects.rs       # Screen effects (shake, flash)
│   ├── particles.rs     # Particle system (8 effect types)
│   ├── player.rs        # Animation player with state machine
│   ├── types.rs         # Animation frames, timing, loop modes
│   └── whammy.rs        # 30 Whammy animation definitions
├── gfx/mod.rs           # Big Board rendering (ggez graphics)
└── ui/mod.rs            # Overlay UI (questions, menus, game over)
```

### Key Architectural Patterns

**ggez EventHandler**: The game implements `ggez::event::EventHandler` trait for the main game loop, providing `update()` and `draw()` methods with proper Context passing.

**Event-Driven Audio**: `GameState::update()` returns `Vec<AudioEvent>` which the main loop passes to `AudioEngine::handle_event()`. Audio is never triggered directly from game logic.

**Hybrid Audio System**: Supports both file-based audio (OGG/WAV) and procedural synthesis fallback at 44.1kHz 16-bit PCM. Per-category volume controls (master, music, effects, voice, audience). Board tones follow an authentic 18-note musical sequence (D4, E4, G4, Bb4...).

**State Machine**: Game flow is controlled by `GamePhase` enum (Start, Questions, Board, GameOver). The `QuestionState` and `ResultState` structs manage sub-states within phases.

**Canvas/Mesh Rendering**: Graphics use ggez Canvas and Mesh pattern for efficient 2D rendering with the wgpu backend.

**Frame-Rate Independence**: All animations use `delta_time` for smooth rendering across different refresh rates.

### Core Data Structures

- `GameState`: Central state containing all game data, the 18-square board, 3 contestants, and phase info
- `BoardSquare`: Contains 3 cycling `Prize` variants that rotate every second
- `Prize`/`PrizeType`: Enum variants for Cash, Prize (physical), Whammy, and Special actions
- `Contestant`: Player data including earned/passed spins, whammies, elimination status
- `WhammyAnimation`: 66+ animation types with catchphrases including holiday specials and elimination animations

### Board Layout

The Big Board is an 18-square perimeter around a center display area. Squares are indexed clockwise from top-left (0) through left column (17). Layout constant `SQUARE_POSITIONS` maps linear indices to (col, row) grid positions.

## Game Rules Implementation

- **Spin Types**: Earned spins (from questions/bonuses) can be passed; passed spins cannot be re-passed
- **Passing**: Pass to leader, or to 2nd place if you're leading
- **Whammy**: Resets score to $0, converts passed spins to earned, 4th Whammy eliminates
- **Question Round**: 4 questions per round; buzz-in correct = 3 spins, multiple choice = 1 spin each
- **Special Squares**: Add-A-One, Double Your Money, Pick a Corner, $2000 or Lose Whammy, Move One Space

## Dependencies

- `ggez 0.9` - Rust game library (wgpu backend, rodio audio)
- `rand 0.9` / `fastrand 2.0` - random number generation for board patterns
- `serde` / `serde_json 1.0` - serialization (save/load ready)
