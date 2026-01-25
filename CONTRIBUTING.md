# Contributing to RatPM

Thank you for your interest in contributing to RatPM!

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Fedora-based Linux system (for testing)
- libdnf5 development libraries
- RPM development libraries

### Building from Source

```bash
git clone https://github.com/SuperfectaOrg/RatPM.git
cd RatPM
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Debug Logging

```bash
RUST_LOG=debug cargo run -- search vim
```

## Code Standards

### Rust Style

- Follow the official Rust style guide
- Run `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Maintain documentation for public APIs

### Commit Messages

Use conventional commit format:

```
type(scope): subject

body

footer
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(cli): add --quiet flag for silent operation

Adds a new global flag to suppress all non-error output.
Useful for scripting and automation.

Closes #42
```

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run `cargo fmt` and `cargo clippy`
7. Commit your changes
8. Push to your fork
9. Open a Pull Request

### Code Review

All submissions require review. We use GitHub Pull Requests for this purpose.

## Architecture Guidelines

### Adding New Commands

1. Add command to `src/cli/mod.rs`
2. Implement command handler in `src/cli/commands.rs`
3. Add integration test in `tests/integration_test.rs`
4. Update documentation in `docs/ratpm.8.md`

### Backend Changes

- All backend changes must maintain compatibility with Fedora
- Do not bypass libdnf5 or RPM APIs
- Maintain transaction atomicity guarantees
- Add appropriate error handling

### Error Handling

- Use `RatpmError` for domain errors
- Use `anyhow::Result` for CLI-level operations
- All errors must include actionable messages
- Errors must map to appropriate exit codes

## Testing

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // test code
    }
}
```

### Integration Tests

Place integration tests in `tests/`:

```rust
#[test]
fn test_command() {
    // test code
}
```

### Test Coverage

Aim for:
- 80%+ coverage for core modules
- 100% coverage for critical paths (transaction execution, lock management)
- All public APIs must have tests

## Documentation

### Code Documentation

```rust
/// Brief description of function
///
/// # Arguments
///
/// * `arg` - Description of argument
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When this function will return an error
pub fn example(arg: String) -> Result<()> {
    // implementation
}
```

### User Documentation

- Update man pages for user-facing changes
- Update README.md for significant features
- Add examples for new functionality

## Security

### Reporting Vulnerabilities

**Do not open public issues for security vulnerabilities.**

Email security reports to: security@superfecta.org

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Security Guidelines

- Always validate user input
- Never execute shell commands with unsanitized input
- Maintain privilege separation
- Use safe Rust patterns (avoid unsafe unless absolutely necessary)
- All file operations must check permissions

## Performance

- Profile before optimizing
- Benchmark performance-critical code
- Avoid unnecessary allocations
- Use streaming for large data sets
- Cache when appropriate

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Update man pages with version
4. Tag release: `git tag -a v1.0.0 -m "Release 1.0.0"`
5. Push tag: `git push origin v1.0.0`
6. Build release artifacts
7. Create GitHub release

## Getting Help

- GitHub Issues: https://github.com/SuperfectaOrg/RatPM/issues
- Discussions: https://github.com/SuperfectaOrg/RatPM/discussions
- Email: team@superfecta.org

## License

By contributing, you agree that your contributions will be licensed under the GPL-3.0-or-later license.
