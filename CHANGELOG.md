# Changelog

All notable changes to Press Your Luck will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.2] - 2025-01-24

### Changed

#### Dependency Updates
- Updated `rand` crate from 0.8.5 to 0.9.2
- Updated GitHub Actions `actions/checkout` from v4 to v6
- Updated GitHub Actions `actions/cache` from v4 to v5

#### API Migration
- Migrated `gen_range()` to `random_range()` for rand 0.9 compatibility
- Updated all random number generation calls throughout codebase

### Technical
- All existing tests pass with updated dependencies
- No breaking changes to game functionality

---

## [2.0.1] - 2025-01-24

### Fixed

#### CI/CD Workflow Fixes
- Fixed incorrect GitHub Action reference (`dtolnay/rust-action` → `dtolnay/rust-toolchain`)
- Added `libudev-dev` dependency to CI workflow (required by ggez for gamepad support)
- All CI jobs now pass: check, clippy, format, build

#### Code Quality (Clippy Compliance)
- Added `#[allow(clippy::upper_case_acronyms)]` for `TNT` and `UFO` variants
- Renamed `Contestant::new()` → `Contestant::create()` (self_named_constructors)
- Renamed `Prize::prize()` → `Prize::physical_prize()` (self_named_constructors)
- Added `#[allow(clippy::too_many_arguments)]` to 8 rendering functions
- Added `#[derive(Default)]` to `AudioEngine` struct
- Fixed documentation comment formatting

### Changed

#### Project Metadata Updates
- Updated author to DoubleGate <parobek@gmail.com>
- Updated GitHub repository description and topics
- Topics: rust, game, gameshow, press-your-luck, whammy, trivia, ggez, retro-gaming, 1980s

---

## [2.0.0] - 2025-01-24

### Changed

#### Framework Migration: macroquad → ggez
- **BREAKING**: Migrated entire codebase from macroquad 0.4 to ggez 0.9
- Resolves RUSTSEC-2025-0035 security vulnerability (macroquad soundness issues)
- ggez provides better long-term sustainability with wgpu backend and rodio audio

#### Architecture Changes
- Renamed `graphics/` module to `gfx/` to avoid namespace collision with `ggez::graphics`
- Implemented ggez `EventHandler` trait pattern for game loop
- Updated audio system to use ggez audio API with Context passing
- Converted rendering to ggez Canvas/Mesh pattern

### Removed
- WebAssembly target support (ggez does not support WASM)
- macroquad and futures dependencies

### Security
- Eliminated RUSTSEC-2025-0035 vulnerability by removing macroquad dependency

---

## [1.1.0] - 2025-01-24

### Added

#### Authentic Whammy Character Redesign
- Completely redesigned Whammy to match original 1983-1986 show character
- Added yellow superhero-style eye mask with pointed edges
- Added yellow flowing cape with wave animation
- Added yellow chest shield with hand-drawn dollar sign ($) emblem
- Added small tuft of hair (3 spikes) instead of devil horns
- Added pointy cartoon feet (authentic to original design)
- Modular drawing system with 7 component functions for maintainability

#### New Whammy Animations (13 additional)
- Holiday animations: ThanksgivingTurkey, ScroogeWhammy, ChristmasTree, ValentineCupid,
  EasterBunny, Leprechaun, FourthOfJuly, HalloweenVampire, OlympicsWhammy, NewYearBaby
- Whammy-out elimination animations: GrimReaper, FiringSquad, AngelWhammy
- Special 4th-Whammy elimination logic using dramatic animations

#### Animation-Responsive Features
- Cape waves differently based on action intensity
- Arms animate for dancing, hammering, jumping, waving
- Legs animate for running, dancing, jumping
- Pupils look different directions based on animation type
- Mouth changes expression (surprised O, evil fangs, innocent smile)

### Changed
- Total Whammy animation types increased from 53 to 66+
- Whammy design corrected from generic devil to authentic show character

### Research Sources
- Press Your Luck Wikipedia and Wikia documentation
- Savage Steve Holland original animator references
- Historical episode analysis

---

## [1.0.0] - 2025-01-24

### Added

#### Core Game Engine
- Complete game state machine with Start, Questions, Board, and GameOver phases
- 18-square Big Board with authentic perimeter layout
- Prize system with 1980s-era values ($500-$5,000 cash, trips, merchandise)
- 53 unique Whammy animation types with original catchphrases
- Special squares: BigBucks, TakeTheLead, Add-A-One, Double Your Money, Pick a Corner
- Trivia question system with 4-answer multiple choice format
- Authentic spin mechanics with exponential deceleration curve
- Event-driven audio system for synchronized sound effects

#### Graphics
- Authentic CRT television color palette with warm 1980s aesthetics
- Chase lights around board perimeter synchronized to spin speed
- LED-style seven-segment score displays with rolling number animations
- Enhanced Whammy character with breathing animation, curved horns, animated eyebrows
- Square flash patterns during board cycling
- CRT overlay effects: scanlines, vignette, phosphor glow, color fringing
- Contestant podiums with metallic trim and name plates
- Round indicator displaying "BIG MONEY!" for Round 2
- PLAY/PASS button visual indicators

#### Audio
- Procedural audio synthesis engine (44.1kHz 16-bit PCM WAV)
- ADSR envelope system for authentic synthesizer tones
- Authentic musical scale matching original show: D, E, G, Bb, D, Ab, F, C
- Board tick sounds during cycling
- Stop "chunk" sound when board halts
- Whammy sad trombone ("wah-wah-waaaah")
- Tension music loop during spinning sequences
- Audience reaction sounds (cheers, gasps)
- Big Bucks and prize win fanfares
- Zero external audio files - all sounds generated at runtime

#### User Interface
- Question display with answer selection highlighting
- Buzz-in timer with visual countdown
- Score display with position rankings (1st, 2nd, 3rd)
- Current player indicator with spin/pass status
- Phase-appropriate UI transitions

### Technical
- Built with Rust and macroquad for cross-platform support
- Optimized release builds with LTO and single codegen unit
- Zero compiler warnings with full clippy compliance
- No external asset dependencies

---

## Version History Summary

| Version | Date | Highlights |
|---------|------|------------|
| 2.0.2 | 2025-01-24 | Dependency updates (rand 0.9.2, actions v6/v5) |
| 2.0.1 | 2025-01-24 | CI/CD fixes, clippy compliance |
| 2.0.0 | 2025-01-24 | Framework migration to ggez 0.9, security fix |
| 1.1.0 | 2025-01-24 | Authentic Whammy redesign, 66+ animations, holiday specials |
| 1.0.0 | 2025-01-24 | Initial release with full game implementation |

[2.0.2]: https://github.com/doublegate/PressYourLuck/releases/tag/v2.0.2
[2.0.1]: https://github.com/doublegate/PressYourLuck/releases/tag/v2.0.1
[2.0.0]: https://github.com/doublegate/PressYourLuck/releases/tag/v2.0.0
[1.1.0]: https://github.com/doublegate/PressYourLuck/releases/tag/v1.1.0
[1.0.0]: https://github.com/doublegate/PressYourLuck/releases/tag/v1.0.0
