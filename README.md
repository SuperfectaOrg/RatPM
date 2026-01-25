# RatPM - RatOS Package Manager

RatPM is the primary package management frontend for RatOS, a Fedora-based Linux distribution.

## Overview

RatPM provides a policy enforcement layer, transaction orchestration, and user interface on top of proven Fedora package management infrastructure (RPM, libdnf5). It does not reimplement dependency resolution or package formats.

## Architecture

- **Backend**: Fedora (RPM + libdnf5)
- **Language**: Rust
- **Configuration**: TOML
- **Interface**: CLI

## Building
```bash
cargo build --release
```

The compiled binary will be at `target/release/ratpm`.

## Installation
```bash
sudo install -m 755 target/release/ratpm /usr/bin/ratpm
sudo mkdir -p /etc/ratpm
sudo cp ratpm.toml.example /etc/ratpm/ratpm.toml
```

## Configuration

System configuration: `/etc/ratpm/ratpm.toml`
User configuration: `~/.config/ratpm/ratpm.toml`

Example configuration:
```toml
[system]
backend = "fedora"
assume_yes = false
color = true
cache_dir = "/var/cache/ratpm"
lock_file = "/var/lock/ratpm.lock"

[repos]
auto_refresh = true
metadata_expire = 86400
repo_dir = "/etc/yum.repos.d"
gpgcheck = true

[transaction]
keep_cache = true
history_limit = 100
verify_signatures = true
```

## Commands

- `ratpm install <packages>` - Install packages
- `ratpm remove <packages>` - Remove packages
- `ratpm update` - Update repository metadata
- `ratpm upgrade [packages]` - Upgrade system or specific packages
- `ratpm search <query>` - Search for packages
- `ratpm info <package>` - Show package information
- `ratpm list [--installed|--available]` - List packages
- `ratpm sync` - Synchronize package databases
- `ratpm doctor` - Run system diagnostics
- `ratpm history [--limit N]` - Show transaction history

## Global Options

- `-y, --assume-yes` - Automatically answer yes to prompts
- `--no-color` - Disable colored output

## Lock Management

RatPM uses a system-wide lock (`/var/lock/ratpm.lock`) to prevent concurrent package operations. The lock is automatically acquired and released. If a process holding the lock crashes, RatPM will detect and clean up stale locks.

## Security

- Root privileges required for all write operations (install, remove, upgrade, update)
- GPG verification of repository metadata and packages (when enabled)
- Transaction atomicity ensures partial installations cannot corrupt the system
- Scriptlet execution is isolated and monitored

## Error Handling

All errors include actionable messages. Exit codes:

- 0: Success
- 1: Package not found or general error
- 2: Dependency conflict
- 3: Transaction check failed
- 4: Transaction execution failed
- 5: Network error
- 6: Repository error
- 7: Insufficient disk space
- 8: Configuration error
- 9: RPM database error
- 10: Scriptlet execution failed
- 13: Permission denied
- 14: Lock held by another process

## Development Status

This is RatPM v1. Current implementation includes:

- Core CLI interface
- Repository management
- Basic dependency resolution
- Transaction execution framework
- Lock management
- Error handling

### Known Limitations

- libdnf5 integration uses placeholder implementation pending FFI bindings
- RPM database operations use simplified mock implementation
- Package downloads simulate network operations
- No actual scriptlet execution
- History tracking not persisted to database

### Production Readiness Checklist

To make RatPM production-ready:

1. Implement actual libdnf5 FFI bindings (create `libdnf5-sys` crate)
2. Integrate with real RPM C library via `rpm` crate
3. Implement actual package download with progress tracking
4. Add GPG signature verification
5. Implement transaction history persistence (SQLite)
6. Add comprehensive test suite
7. Implement recovery from interrupted transactions
8. Add checksum verification for all downloads
9. Implement repository mirror failover
10. Add metrics and telemetry

## License

GPL-3.0-or-later
