# Contributing to Press Your Luck

Thank you for your interest in contributing to this authentic recreation of the 1983-1986 CBS game show!

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Linux: `libasound2-dev` (ALSA development libraries)
- macOS: No additional dependencies
- Windows: No additional dependencies

### Building

```bash
# Clone the repository
git clone https://github.com/doublegate/PressYourLuck.git
cd PressYourLuck

# Build in debug mode
cargo build

# Build optimized release
cargo build --release

# Run the game
cargo run --release
```

## Development Guidelines

### Code Style

- Follow standard Rust conventions (`cargo fmt`)
- Ensure zero clippy warnings (`cargo clippy -- -D warnings`)
- All code must compile without warnings

### Authenticity First

This project prioritizes faithfulness to the original 1983-1986 CBS television show. When contributing:

1. **Research the original show** - Watch episodes, study the Big Board layout, listen to sound effects
2. **Match the era** - Prize values, graphics style, and music should reflect mid-1980s television
3. **Preserve game mechanics** - The original rules (passing spins, 4 Whammies, etc.) must be maintained

### Areas for Contribution

#### High Priority
- Additional trivia questions (1980s pop culture, general knowledge)
- More Whammy animation variations
- Sound effect refinements
- Cross-platform testing (Windows, macOS, WASM)

#### Medium Priority
- Accessibility features
- Configuration options
- Save/load game state
- Network multiplayer

#### Research Needed
- Original show episode analysis
- Authentic prize catalog from 1983-1986
- Original musical cues and stingers

### Procedural Generation Philosophy

This project generates all assets procedurally:
- **Audio**: 44.1kHz 16-bit PCM synthesis with ADSR envelopes
- **Graphics**: Vector-style rendering via macroquad
- **No external assets**: Everything is code-generated

When adding features, maintain this philosophy. External asset files should be avoided.

## Pull Request Process

1. **Fork the repository** and create a feature branch
2. **Make your changes** following the code style guidelines
3. **Test thoroughly** - ensure `cargo build` and `cargo clippy` pass cleanly
4. **Write a clear PR description** explaining what and why
5. **Reference any issues** being addressed

### Commit Message Format

Follow conventional commits:

```
feat: add new Whammy animation for "bowling strike"
fix: correct LED score display digit alignment
docs: update README with macOS build instructions
refactor: extract audio synthesis into separate module
```

## Reporting Issues

When reporting bugs, please include:

1. **Operating system and version**
2. **Rust version** (`rustc --version`)
3. **Steps to reproduce**
4. **Expected vs actual behavior**
5. **Console output or error messages**

## Code of Conduct

- Be respectful and constructive
- Focus on the code, not the person
- Welcome newcomers and help them contribute
- Keep discussions on-topic

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Open a GitHub issue for questions about contributing. We're happy to help!
