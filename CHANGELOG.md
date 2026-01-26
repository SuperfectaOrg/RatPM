# Changelog

All notable changes to RatPM will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-01-26

### Added
- Initial release of RatPM
- Core package management commands: install, remove, update, upgrade
- Search and info commands for package discovery
- List command for viewing installed and available packages
- Sync command for database synchronization
- Doctor command for system diagnostics
- History command for viewing transaction history
- TOML-based configuration system
- System-wide lock management to prevent concurrent operations
- Repository management with .repo file parsing
- Transaction execution with atomicity guarantees
- GPG verification support for packages and metadata
- Colored terminal output with --no-color option
- Systemd timer for automatic repository metadata refresh
- Comprehensive error handling with actionable messages
- Man pages for ratpm(8) and ratpm.toml(5)
- Integration with Fedora's RPM and libdnf5 infrastructure

### Security
- Root privilege enforcement for write operations
- GPG signature verification enabled by default
- Lock file prevents concurrent package operations
- Transaction rollback on failure

### Known Limitations
- libdnf5 integration uses placeholder implementation pending full FFI bindings
- RPM database operations use simplified mock implementation
- Package downloads simulate network operations
- History tracking not yet persisted to SQLite database

[Unreleased]: https://github.com/SuperfectaOrg/RatPM/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/SuperfectaOrg/RatPM/releases/tag/v1.0.0
