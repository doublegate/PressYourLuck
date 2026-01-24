# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Press Your Luck is an authentic recreation of the 1983-1986 CBS game show, built in Rust using the macroquad game framework. The game features procedurally generated audio (no external files), vector-based graphics, and implements the complete game rules including question rounds, the Big Board, passing mechanics, and 4-Whammy elimination.

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

Requires ALSA development libraries:
```bash
# Ubuntu/Debian
sudo apt install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel

# Arch/CachyOS
sudo pacman -S alsa-lib
```

## Architecture

The codebase follows a modular architecture with clear separation between game logic, rendering, audio, and UI:

```
src/
├── main.rs      # Entry point, game loop, input processing
├── game/mod.rs  # Game state, rules, all game logic
├── audio/mod.rs # Procedural sound synthesis (WAV generation)
├── graphics/mod.rs # Big Board rendering, animations
└── ui/mod.rs    # Overlay UI (questions, menus, game over)
```

### Key Architectural Patterns

**Event-Driven Audio**: `GameState::update()` returns `Vec<AudioEvent>` which the main loop passes to `AudioEngine::handle_event()`. Audio is never triggered directly from game logic.

**Procedural Audio**: All sounds are synthesized mathematically at 44.1kHz 16-bit PCM using waveform generation with ADSR envelopes. No external audio files are needed. Board tones follow an authentic 18-note musical sequence.

**State Machine**: Game flow is controlled by `GamePhase` enum (Start, Questions, Board, GameOver). The `QuestionState` and `ResultState` structs manage sub-states within phases.

**Frame-Rate Independence**: All animations use `delta_time` for smooth rendering across different refresh rates.

### Core Data Structures

- `GameState`: Central state containing all game data, the 18-square board, 3 contestants, and phase info
- `BoardSquare`: Contains 3 cycling `Prize` variants that rotate every second
- `Prize`/`PrizeType`: Enum variants for Cash, Prize (physical), Whammy, and Special actions
- `Contestant`: Player data including earned/passed spins, whammies, elimination status
- `WhammyAnimation`: 12 animation types with catchphrases (Pogo, TNT, Hammer, etc.)

### Board Layout

The Big Board is an 18-square perimeter around a center display area. Squares are indexed clockwise from top-left (0) through left column (17). Layout constant `SQUARE_POSITIONS` maps linear indices to (col, row) grid positions.

## Game Rules Implementation

- **Spin Types**: Earned spins (from questions/bonuses) can be passed; passed spins cannot be re-passed
- **Passing**: Pass to leader, or to 2nd place if you're leading
- **Whammy**: Resets score to $0, converts passed spins to earned, 4th Whammy eliminates
- **Question Round**: 4 questions per round; buzz-in correct = 3 spins, multiple choice = 1 spin each
- **Special Squares**: Add-A-One, Double Your Money, Pick a Corner, $2000 or Lose Whammy, Move One Space

## Dependencies

- `macroquad 0.4` with audio feature - cross-platform 2D game framework
- `rand/fastrand` - random number generation for board patterns
- `serde/serde_json` - serialization (save/load ready)
