# Testing Guide

This document describes the testing strategy and practices for RatPM.

## Testing Philosophy

RatPM follows a multi-layered testing approach:

1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test component interactions
3. **System Tests** - Test end-to-end workflows (future)
4. **Performance Tests** - Benchmarks for critical paths

## Running Tests

### All Tests
```bash
cargo test
```

### Specific Test File
```bash
cargo test --test integration_test
```

### Specific Test Function
```bash
cargo test test_lock_acquire_and_release
```

### With Output
```bash
cargo test -- --nocapture
```

### With Logging
```bash
RUST_LOG=debug cargo test -- --nocapture
```

### Release Mode
```bash
cargo test --release
```

## Test Organization

### Unit Tests

Located in the same file as the code:
```rust
// src/core/transaction.rs

impl Transaction {
    pub fn new() -> Self {
        // implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_new() {
        let t = Transaction::new();
        assert!(t.is_empty());
    }
}
```

### Integration Tests

Located in `tests/`:
```
tests/
├── integration_test.rs
├── config_test.rs
├── lock_test.rs
├── transaction_test.rs
├── backend_test.rs
└── error_test.rs
```

### Benchmarks

Located in `benches/`:
```
benches/
├── transaction_bench.rs
└── config_bench.rs
```

## Test Coverage

### Measuring Coverage
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Open report
open tarpaulin-report.html
```

### Coverage Targets

- Overall: 80%+
- Core modules: 90%+
- Critical paths (locks, transactions): 100%

### Current Coverage

Run `cargo tarpaulin` to see current coverage statistics.

## Writing Tests

### Unit Test Template
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
    
    #[test]
    fn test_error_case() {
        let result = function_that_fails();
        assert!(result.is_err());
    }
    
    #[test]
    #[should_panic(expected = "error message")]
    fn test_panic_case() {
        function_that_panics();
    }
}
```

### Integration Test Template
```rust
use std::process::Command;

#[test]
fn test_command_execution() {
    let output = Command::new("cargo")
        .args(&["run", "--", "command", "args"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("expected output"));
}
```

### Benchmark Template
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_function(c: &mut Criterion) {
    c.bench_function("function_name", |b| {
        b.iter(|| {
            black_box(function_under_test(black_box(input)))
        })
    });
}

criterion_group!(benches, bench_function);
criterion_main!(benches);
```

## Test Best Practices

### 1. Test One Thing
```rust
// Good
#[test]
fn test_transaction_is_empty_when_new() {
    let t = Transaction::new();
    assert!(t.is_empty());
}

#[test]
fn test_transaction_not_empty_after_add() {
    let mut t = Transaction::new();
    t.add_install(pkg, 100);
    assert!(!t.is_empty());
}

// Bad - testing multiple things
#[test]
fn test_transaction() {
    let mut t = Transaction::new();
    assert!(t.is_empty());
    t.add_install(pkg, 100);
    assert!(!t.is_empty());
    assert_eq!(t.download_size, 100);
}
```

### 2. Use Descriptive Names
```rust
// Good
#[test]
fn test_lock_prevents_concurrent_access() { }

#[test]
fn test_config_validation_fails_for_invalid_backend() { }

// Bad
#[test]
fn test1() { }

#[test]
fn test_lock() { }
```

### 3. Arrange-Act-Assert
```rust
#[test]
fn test_example() {
    // Arrange - set up test data
    let config = Config::default();
    
    // Act - perform the operation
    let result = validate_config(&config);
    
    // Assert - verify the result
    assert!(result.is_ok());
}
```

### 4. Clean Up Resources
```rust
use tempfile::TempDir;

#[test]
fn test_with_temp_dir() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.toml");
    
    // Test code using file_path
    
    // temp_dir automatically cleaned up when it goes out of scope
}
```

### 5. Test Error Cases
```rust
#[test]
fn test_install_requires_root() {
    let context = Context::new(Config::default()).unwrap();
    
    if !nix::unistd::geteuid().is_root() {
        let result = context.require_root();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RatpmError::PermissionDenied));
    }
}
```

### 6. Use Test Fixtures
```rust
fn create_test_package() -> PackageSpec {
    PackageSpec::new(
        "test-package".to_string(),
        "1.0.0".to_string(),
        "x86_64".to_string(),
        "test-repo".to_string(),
    )
}

#[test]
fn test_using_fixture() {
    let pkg = create_test_package();
    // Use pkg in test
}
```

## Testing Specific Components

### Testing Lock Manager
```rust
#[test]
fn test_lock_acquisition() {
    let temp_file = NamedTempFile::new().unwrap();
    let lock_manager = FileLockManager::new(temp_file.path().to_path_buf());
    
    let guard = lock_manager.acquire();
    assert!(guard.is_ok());
}
```

### Testing Configuration
```rust
#[test]
fn test_config_loading() {
    let toml = r#"
        [system]
        backend = "fedora"
    "#;
    
    let config: Config = toml::from_str(toml).unwrap();
    assert_eq!(config.system.backend, "fedora");
}
```

### Testing Transactions
```rust
#[test]
fn test_transaction_add_install() {
    let mut t = Transaction::new();
    let pkg = create_test_package();
    
    t.add_install(pkg, 1_000_000);
    
    assert_eq!(t.total_packages(), 1);
    assert_eq!(t.download_size, 1_000_000);
}
```

### Testing CLI Commands
```rust
#[test]
fn test_search_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "search", "vim"])
        .output()
        .unwrap();
    
    assert!(output.status.success());
}
```

## Continuous Integration

Tests run automatically on:
- Every push to main/develop
- Every pull request
- Nightly (for extended test suite)

See `.github/workflows/ci.yml` for CI configuration.

## Performance Testing

### Running Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench transaction

# Save baseline
cargo bench -- --save-baseline my-baseline

# Compare with baseline
cargo bench -- --baseline my-baseline
```

### Benchmark Organization
```
benches/
├── transaction_bench.rs  # Transaction operations
└── config_bench.rs       # Configuration parsing
```

## Common Testing Patterns

### Testing with Temporary Files
```rust
use tempfile::{TempDir, NamedTempFile};

#[test]
fn test_with_temp_file() {
    let temp_file = NamedTempFile::new().unwrap();
    // Use temp_file.path()
}

#[test]
fn test_with_temp_dir() {
    let temp_dir = TempDir::new().unwrap();
    // Use temp_dir.path()
}
```

### Testing Async Code
```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Testing with Mocks
```rust
#[test]
fn test_with_mock_backend() {
    let mock_backend = MockBackend::new();
    let context = Context::with_backend(mock_backend);
    
    // Test using mocked backend
}
```

## Debugging Tests

### Print Debug Output
```bash
cargo test -- --nocapture
```

### Run Single Test with Logging
```bash
RUST_LOG=debug cargo test test_name -- --nocapture
```

### Run Tests with Backtrace
```bash
RUST_BACKTRACE=1 cargo test
```

## Test Maintenance

### Keeping Tests Fast

- Use mocks for slow operations
- Run unit tests frequently
- Save integration tests for CI
- Use `cargo test --lib` for library tests only

### Keeping Tests Reliable

- Avoid timing-dependent tests
- Use deterministic test data
- Clean up resources properly
- Avoid global state

### Keeping Tests Maintainable

- Follow naming conventions
- Keep tests simple
- Avoid test interdependencies
- Document complex test setups

## Resources

- [Rust Book: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
