# Changelog

All notable changes to Press Your Luck will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
| 1.0.0 | 2025-01-24 | Initial release with full game implementation |

[1.0.0]: https://github.com/doublegate/PressYourLuck/releases/tag/v1.0.0
