# Press Your Luck

An authentic recreation of the 1983-1986 CBS game show hosted by Peter Tomarken,
with Rod Roddy announcing and Whammy animations by Savage Steve Holland.

## Features

- **Authentic 18-square Big Board** with cycling prizes and chase lights
- **3-player format** with question rounds and board rounds
- **66+ unique Whammy animations** with classic catchphrases (regular, holiday, and elimination specials)
- **Authentic Whammy character** - Yellow cape, eye mask, and dollar sign emblem (researched from original show)
- **Authentic musical tones** (D, E, G, B♭, D, A♭, F, C...)
- **Special squares**: Add-A-One, Double Your Money, Pick a Corner, Big Bucks, Take the Lead
- **Proper passing rules** and 4-Whammy elimination with special "Whammy-out" animations
- **Procedurally generated audio** (no external files needed)
- **CRT visual effects**: Scanlines, vignette, phosphor glow, color fringing
- **LED-style score displays** with rolling number animations
- **Tension music** during board spins with authentic deceleration curve

## Building

### Prerequisites

- Rust 1.70 or later
- On Linux: `libasound2-dev` and `libudev-dev` (ALSA and udev development libraries)

```bash
# Ubuntu/Debian
sudo apt install libasound2-dev libudev-dev

# Fedora
sudo dnf install alsa-lib-devel libudev-devel

# Arch/CachyOS
sudo pacman -S alsa-lib systemd-libs
```

### Development Build

```bash
cargo build
cargo run
```

### Release Build (Optimized)

```bash
cargo build --release
./target/release/press-your-luck
```

## Controls

| Key       | Action                                    |
|-----------|-------------------------------------------|
| SPACE     | Start/Stop spin, Continue                 |
| P         | Pass spins to opponent                    |
| B         | Buzz in during questions (3 spins)        |
| 1-4       | Select answer or corner                   |
| ENTER     | Confirm selection                         |
| ESC       | Quit game                                 |

## Game Rules

### Question Round
- 4 questions per round
- **Buzz-in correct** = 3 spins (only one player gets them)
- **Multiple choice correct** = 1 spin (all players get a chance)

### Board Round
- Use your spins on the Big Board
- **Cash**: Add to your score
- **Prize**: Win a physical prize (adds value to score)
- **Special**: Trigger special actions like Add-A-One or Double Your Money
- **WHAMMY**: Lose ALL your money and get a Whammy!

### Passing Rules
- You can pass **earned spins** (not passed spins) to opponents
- Pass to the **leader**
- If you're leading, pass to **2nd place**

### Whammy Rules
- Landing on a Whammy resets your score to $0
- Passed spins convert to earned spins when you hit a Whammy
- **4 Whammies = You're OUT!**

## Project Structure

```
src/
├── main.rs      # Entry point, ggez EventHandler, game loop
├── game/
│   └── mod.rs   # Game state, rules, and logic
├── audio/
│   └── mod.rs   # Procedural sound synthesis (ggez audio)
├── gfx/
│   └── mod.rs   # Rendering and animations (ggez graphics)
└── ui/
    └── mod.rs   # User interface overlays
```

## Technical Details

- **Framework**: ggez 0.9 (Rust game library inspired by LÖVE2D, uses wgpu/rodio)
- **Audio**: Procedurally generated WAV at 44.1kHz, 16-bit mono
- **Graphics**: Vector-based rendering with Canvas/Mesh pattern
- **Platform**: Linux (primary), Windows, macOS

## The Whammy Character

The Whammy was designed by animator **Savage Steve Holland**, who sketched the original
concept on a napkin. Contrary to popular belief, the Whammy is NOT a devil - it's a
mischievous cartoon anti-hero featuring:

- **Yellow superhero-style eye mask** with pointed edges
- **Yellow flowing cape** that waves during animations
- **Yellow chest shield** with a dollar sign ($) emblem
- **Small tuft of hair** (not devil horns!)
- **Pointy feet** and expressive cartoon limbs
- **Mischievous grin** that changes based on the animation

This implementation includes 66+ authentic animation types including holiday specials
(Thanksgiving, Christmas, Halloween, etc.) and dramatic "Whammy-out" elimination animations.

## Historical Notes

Press Your Luck aired on CBS from September 19, 1983 to September 26, 1986.
The show became famous for the "Big Bucks" and "No Whammies!" catchphrases,
and the Michael Larson incident where a contestant memorized the board patterns
and won $110,237 in a single episode (1984).

## License

MIT License - See LICENSE file for details.

---

*"Big Bucks! No Whammies! STOP!"*
