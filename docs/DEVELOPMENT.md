# Development Guide

This guide covers the development workflow, coding standards, and best practices for contributing to RatPM.

## Getting Started

### Prerequisites

- Fedora 38+ or compatible RHEL-based system
- Rust 1.70 or later
- Git
- Basic understanding of package management concepts

### Initial Setup
```bash
# Clone the repository
git clone https://github.com/SuperfectaOrg/RatPM.git
cd RatPM

# Run the setup script
./scripts/dev-setup.sh

# Build the project
cargo build

# Run tests
cargo test
```

## Project Structure
```
RatPM/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   ├── cli/                 # CLI interface
│   ├── core/                # Core business logic
│   ├── backend/             # Backend implementations
│   └── config/              # Configuration management
├── libdnf5-sys/             # FFI bindings
├── tests/                   # Integration tests
├── benches/                 # Benchmarks
├── docs/                    # Documentation
├── scripts/                 # Development scripts
├── systemd/                 # Systemd units
└── packaging/               # RPM spec files
```

## Development Workflow

### Creating a Feature
```bash
# Create a feature branch
git checkout -b feat/my-feature

# Make changes
vim src/...

# Run tests frequently
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy

# Commit changes
git commit -m "feat(scope): description"

# Push and create PR
git push origin feat/my-feature
```

### Running in Debug Mode
```bash
# Run with debug logging
RUST_LOG=debug cargo run -- install vim

# Run with trace logging (very verbose)
RUST_LOG=trace cargo run -- search neovim

# Run specific command
cargo run -- --help
```

### Testing Changes
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_lock_acquire_and_release

# Run integration tests only
cargo test --test integration_test

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Coding Standards

### Rust Style

Follow the official Rust style guide:
```bash
# Format all code
cargo fmt

# Check formatting without changing
cargo fmt -- --check
```

### Linting
```bash
# Run clippy with all warnings
cargo clippy --all-targets --all-features -- -D warnings

# Fix automatically where possible
cargo clippy --fix
```

### Documentation

Document all public APIs:
```rust
/// Search for packages matching the query.
///
/// # Arguments
///
/// * `query` - Search string to match against package names and descriptions
///
/// # Returns
///
/// A vector of matching packages, sorted by relevance
///
/// # Errors
///
/// Returns `RatpmError::RepoUnavailable` if repositories cannot be accessed
///
/// # Examples
///
/// ```
/// let packages = backend.search("vim")?;
/// ```
pub fn search(&self, query: &str) -> Result<Vec<Package>> {
    // implementation
}
```

### Error Handling

Use the error hierarchy consistently:
```rust
// Domain errors - use RatpmError
if !self.is_root {
    return Err(RatpmError::PermissionDenied);
}

// CLI level - use anyhow::Result
pub fn execute(context: &mut Context) -> Result<()> {
    context.require_root()
        .context("Root privileges required")?;
    // ...
}

// Add context to errors
file.read_to_string(&mut contents)
    .context("Failed to read configuration file")?;
```

### Naming Conventions

- **Functions**: `snake_case`
- **Types/Structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`
```rust
const MAX_RETRIES: u32 = 3;

pub struct PackageSpec {
    pub name: String,
}

pub fn resolve_dependencies(pkg: &PackageSpec) -> Result<Vec<PackageSpec>> {
    // implementation
}
```

## Module Guidelines

### CLI Layer

- Never call backend directly
- Always go through Core layer
- Handle all user interaction
- Format output consistently
```rust
// Good
pub fn execute(context: &mut Context, packages: Vec<String>) -> Result<()> {
    let transaction = context.backend_mut().resolve_install(&packages)?;
    // ...
}

// Bad - bypassing Core
pub fn execute(packages: Vec<String>) -> Result<()> {
    let backend = FedoraBackend::new()?;  // DON'T DO THIS
    // ...
}
```

### Core Layer

- No direct user interaction
- No CLI argument parsing
- Business logic only
- Coordinate backend operations
```rust
// Good
impl Context {
    pub fn require_root(&self) -> Result<(), RatpmError> {
        if !self.is_root {
            return Err(RatpmError::PermissionDenied);
        }
        Ok(())
    }
}

// Bad - formatting output in Core
impl Context {
    pub fn require_root(&self) -> Result<(), RatpmError> {
        if !self.is_root {
            println!("Error: Root required");  // DON'T DO THIS
            return Err(RatpmError::PermissionDenied);
        }
        Ok(())
    }
}
```

### Backend Layer

- Abstract external dependencies
- No policy decisions
- Pure data operations
- Clear error reporting
```rust
// Good
pub fn search(&self, query: &str) -> Result<Vec<Package>> {
    let results = self.repos.search(query)?;
    Ok(results)
}

// Bad - making policy decisions
pub fn search(&self, query: &str) -> Result<Vec<Package>> {
    if query.len() < 3 {  // DON'T DO THIS - policy belongs in Core
        return Err(RatpmError::InvalidInput);
    }
    // ...
}
```

## Testing Guidelines

### Unit Tests

Place in the same file as the code:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_empty() {
        let transaction = Transaction::new();
        assert!(transaction.is_empty());
    }
    
    #[test]
    fn test_transaction_add_install() {
        let mut transaction = Transaction::new();
        let pkg = PackageSpec::new(
            "vim".to_string(),
            "9.0.0".to_string(),
            "x86_64".to_string(),
            "fedora".to_string(),
        );
        transaction.add_install(pkg, 5_000_000);
        assert_eq!(transaction.total_packages(), 1);
    }
}
```

### Integration Tests

Place in `tests/`:
```rust
#[test]
fn test_install_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "install", "vim"])
        .output()
        .expect("Failed to execute");
    
    // Assertions
}
```

### Test Coverage

Aim for:
- 80%+ overall coverage
- 100% coverage for critical paths
- All public APIs tested
```bash
# Install coverage tools
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Debugging

### Using lldb/gdb
```bash
# Build with debug symbols
cargo build

# Run in debugger
rust-lldb target/debug/ratpm

# Set breakpoint
(lldb) b ratpm::core::context::Context::new
(lldb) run install vim
```

### Logging

Use the `tracing` crate:
```rust
use tracing::{debug, info, warn, error};

pub fn install(&mut self, packages: &[String]) -> Result<()> {
    info!("Installing {} packages", packages.len());
    debug!("Package list: {:?}", packages);
    
    // ...
    
    warn!("Repository metadata is stale");
    
    // ...
}
```

Run with logging:
```bash
RUST_LOG=ratpm=debug cargo run -- install vim
RUST_LOG=ratpm::core=trace cargo run -- install vim
```

## Performance Profiling

### CPU Profiling
```bash
# Install profiler
cargo install flamegraph

# Generate flamegraph
cargo flamegraph -- install vim

# Open flamegraph.svg in browser
```

### Memory Profiling
```bash
# Use valgrind
valgrind --tool=massif target/release/ratpm install vim

# Analyze results
ms_print massif.out.*
```

### Benchmarking
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench transaction_bench

# Compare with baseline
cargo bench -- --save-baseline my-baseline
# Make changes
cargo bench -- --baseline my-baseline
```

## Documentation

### Building Documentation
```bash
# Build library documentation
cargo doc --no-deps --open

# Build all documentation including dependencies
cargo doc --open
```

### Man Pages
```bash
# Requires pandoc
pandoc -s -t man docs/ratpm.8.md -o docs/ratpm.8
pandoc -s -t man docs/ratpm.toml.5.md -o docs/ratpm.toml.5

# View man page
man docs/ratpm.8
```

## Common Tasks

### Adding a New Command

1. Add to `Commands` enum in `src/cli/mod.rs`
2. Implement handler in `src/cli/commands.rs`
3. Add integration test in `tests/integration_test.rs`
4. Update man page in `docs/ratpm.8.md`
5. Update CHANGELOG.md

### Adding a New Configuration Option

1. Add field to appropriate struct in `src/config/mod.rs`
2. Add default value function
3. Update example config in `ratpm.toml.example`
4. Update man page in `docs/ratpm.toml.5.md`
5. Add validation if needed

### Adding a New Error Type

1. Add variant to `RatpmError` in `src/core/errors.rs`
2. Implement error message
3. Map to exit code in `exit_code()` method
4. Document when this error occurs

## Troubleshooting

### Build Failures
```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for issues
cargo check
```

### Test Failures
```bash
# Run failing test with output
cargo test test_name -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test test_name
```

### Dependency Issues
```bash
# Check dependency tree
cargo tree

# Audit for security issues
cargo audit

# Update specific dependency
cargo update -p package_name
```

## Release Process

See CONTRIBUTING.md for the full release process.

Quick checklist:
1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Update man pages with new version
4. Run full test suite
5. Build release binary
6. Tag release
7. Create GitHub release

## Getting Help

- GitHub Discussions: https://github.com/SuperfectaOrg/RatPM/discussions
- GitHub Issues: https://github.com/SuperfectaOrg/RatPM/issues
- Email: team@superfecta.org

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [RPM Documentation](https://rpm.org/documentation.html)
- [Fedora Packaging Guidelines](https://docs.fedoraproject.org/en-US/packaging-guidelines/)
