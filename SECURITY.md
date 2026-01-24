# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

This is a single-player game with no network functionality, so the attack surface is minimal. However, if you discover a security issue:

1. **Do not open a public issue**
2. **Email the maintainer** or use GitHub's private vulnerability reporting
3. **Include details**: description, steps to reproduce, potential impact

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Fix (if applicable)**: Depending on severity

## Security Considerations

### Current Architecture

- **No network access**: The game runs entirely offline
- **No file system writes**: No save files or configuration persistence (yet)
- **No external assets**: All graphics and audio are procedurally generated
- **No user input persistence**: Player names and scores are session-only

### Dependencies

This project uses well-maintained Rust crates:
- `macroquad` - Cross-platform game library
- `rand`/`fastrand` - Random number generation
- `serde`/`serde_json` - Serialization (for future save/load)
- `futures` - Async runtime

Run `cargo audit` to check for known vulnerabilities in dependencies.

## Best Practices for Contributors

- Keep dependencies updated
- Run `cargo clippy` to catch potential issues
- Avoid `unsafe` code unless absolutely necessary
- No hardcoded credentials or sensitive data
