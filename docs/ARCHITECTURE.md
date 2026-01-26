# RatPM Architecture

## Overview

RatPM is structured as a layered system with clear separation of concerns. This document describes the architectural decisions and component interactions.

## System Layers
```
┌─────────────────────────────────────────┐
│            CLI Layer                     │
│  (Argument parsing, user interaction)    │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│           Core Layer                     │
│  (Business logic, transactions, locks)   │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│         Backend Layer                    │
│    (Fedora integration via libdnf5)      │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│       System Layer                       │
│         (RPM, libdnf5)                   │
└─────────────────────────────────────────┘
```

## Component Responsibilities

### CLI Layer (`src/cli/`)

**Purpose:** User interface and command dispatching.

**Components:**
- `mod.rs`: Argument parsing with clap
- `commands.rs`: Command implementations
- `output.rs`: Formatted output and user feedback

**Responsibilities:**
- Parse command-line arguments
- Validate user input
- Dispatch to core layer
- Format and display output
- Handle interactive confirmations

**Does NOT:**
- Perform package operations directly
- Access RPM database
- Manage locks

### Core Layer (`src/core/`)

**Purpose:** Business logic and orchestration.

**Components:**
- `context.rs`: Global runtime state
- `transaction.rs`: Transaction modeling
- `resolver.rs`: Dependency resolution abstraction
- `lock.rs`: Concurrency control
- `errors.rs`: Error taxonomy

**Responsibilities:**
- Maintain application state
- Enforce business rules
- Coordinate backend operations
- Manage system-wide locks
- Transaction lifecycle management

**Does NOT:**
- Parse CLI arguments
- Call libdnf5 directly (uses backend)
- Format user output

### Backend Layer (`src/backend/fedora/`)

**Purpose:** Integration with Fedora package infrastructure.

**Components:**
- `mod.rs`: Backend interface
- `libdnf.rs`: libdnf5 FFI wrapper
- `rpm.rs`: RPM database operations
- `repos.rs`: Repository management
- `transaction.rs`: Transaction execution
- `types.rs`: Domain types

**Responsibilities:**
- Abstract libdnf5 complexity
- Execute RPM transactions
- Manage repository metadata
- Package search and queries
- Download orchestration

**Does NOT:**
- Make policy decisions
- Handle user interaction
- Manage application-level locks (uses system locks only)

### Configuration (`src/config/`)

**Purpose:** Configuration management.

**Components:**
- `mod.rs`: Config loading and validation
- `schema.rs`: Type definitions

**Responsibilities:**
- Load TOML configuration
- Merge system and user configs
- Validate settings
- Provide defaults

## Data Flow

### Install Command Flow
```
User: ratpm install neovim
         ↓
CLI: Parse args → validate → dispatch
         ↓
Core: Check root → acquire lock → create context
         ↓
Backend: Search package → resolve dependencies
         ↓
Core: Build transaction object
         ↓
CLI: Display summary → confirm with user
         ↓
Backend: Download packages → verify signatures
         ↓
Backend: Execute RPM transaction
         ↓
Core: Record history → release lock
         ↓
CLI: Display success message
```

### Error Flow
```
Error occurs in Backend
         ↓
Backend: Wraps in RatpmError
         ↓
Core: Propagates up (anyhow::Result)
         ↓
CLI: Formats error message
         ↓
CLI: Maps to exit code
         ↓
CLI: Displays to stderr
```

## Concurrency Model

### Lock Hierarchy

1. **System Lock** (`/var/lock/ratpm.lock`)
   - Prevents concurrent RatPM instances
   - Acquired by Core layer
   - Released on drop (RAII)

2. **RPM Database Lock**
   - Managed by librpm internally
   - Prevents concurrent RPM operations
   - Never explicitly managed by RatPM

### Lock Acquisition Rules

- Always acquire in order: System → RPM
- Never hold locks during user interaction
- Locks are file-based (flock)
- Stale locks cleaned up automatically
- Timeout after 30 seconds

## Transaction Semantics

### Transaction States
```
Pending → Resolving → Downloading → Verifying → Testing → Executing → Complete
                                                              ↓
                                                           Failed
```

### Atomicity Guarantees

- All-or-nothing execution
- Rollback on failure
- No partial installations
- Transaction history recorded

### Failure Recovery

1. **Download Failure**: Retry with mirrors, fail if all exhausted
2. **Verification Failure**: Abort transaction, report to user
3. **RPM Failure**: Rollback transaction, restore state
4. **Scriptlet Failure**: Mark package as broken, continue with caution

## Security Model

### Privilege Separation

- **Read operations**: Can run as unpriviled user
  - search, info, list, history
- **Write operations**: Require root
  - install, remove, update, upgrade, sync

### Trust Chain
```
Repository Metadata (GPG signed)
         ↓
Package Checksums (in metadata)
         ↓
Package RPM (GPG signed)
         ↓
Installation (verified)
```

### Security Checks

1. Repository metadata signature
2. Package checksum verification
3. Package GPG signature
4. File integrity checks
5. Scriptlet sandboxing (future)

## Backend Abstraction

### Why Abstract libdnf5?

- Isolate FFI complexity
- Enable testing without libdnf5
- Future multi-backend support
- Clear API boundaries

### Backend Interface
```rust
pub trait PackageBackend {
    fn search(&self, query: &str) -> Result<Vec<Package>>;
    fn resolve_install(&self, packages: &[String]) -> Result<Transaction>;
    fn execute(&mut self, transaction: Transaction) -> Result<()>;
    // ...
}
```

## Performance Considerations

### Caching Strategy

- Repository metadata: Cached with TTL (24h default)
- Downloaded packages: Kept in cache if configured
- Query results: Not cached (always fresh)

### Memory Usage

- Streaming for large downloads
- Lazy loading of repository data
- Package metadata loaded on-demand

### Network Optimization

- Parallel downloads (future)
- Mirror failover
- Resume interrupted downloads (future)

## Testing Strategy

### Unit Tests

- Test individual functions in isolation
- Mock backend interactions
- Focus on business logic

### Integration Tests

- Test command execution end-to-end
- Use real configuration files
- Verify file system changes

### System Tests (Future)

- Test in VM or container
- Real RPM operations
- Full transaction testing

## Extension Points

### Future Backend Support

Interface designed to support:
- APT backend (Debian/Ubuntu)
- Pacman backend (Arch)
- Zypper backend (openSUSE)

### Plugin System (Future)

- Pre/post transaction hooks
- Custom repository types
- Alternative resolvers

## Configuration Hierarchy
```
Defaults (hardcoded)
    ↓
/etc/ratpm/ratpm.toml (system)
    ↓
~/.config/ratpm/ratpm.toml (user)
    ↓
Command-line flags (highest priority)
```

## Logging and Debugging

### Log Levels

- **ERROR**: Fatal errors, transaction failures
- **WARN**: Non-fatal issues, deprecated features
- **INFO**: Normal operations, transaction summaries
- **DEBUG**: Detailed operation logs
- **TRACE**: FFI calls, low-level details

### Debug Mode
```bash
RUST_LOG=debug ratpm install vim
```

Outputs:
- Lock acquisition/release
- Repository queries
- Dependency resolution steps
- Download progress
- RPM transaction details

## Directory Structure
```
/etc/ratpm/
  └── ratpm.toml              # System configuration

/var/cache/ratpm/
  ├── packages/               # Downloaded RPMs
  └── fedora/                 # Repository metadata cache
      └── primary.cache

/var/lib/ratpm/
  └── history.db              # Transaction history (future)

/var/lock/
  └── ratpm.lock              # System-wide lock file
```

## Dependencies

### Runtime Dependencies

- `rpm` - RPM package manager
- `libdnf5` - Dependency resolver
- `systemd` - Timer for auto-refresh

### Build Dependencies

- `rust` >= 1.70
- `cargo`
- `gcc`
- `rpm-devel`
- `libdnf5-devel`

## Performance Targets

- Install command: < 5s (excluding download)
- Search command: < 1s
- Lock acquisition: < 30s (timeout)
- Memory usage: < 100MB (typical)
- Binary size: < 10MB (stripped)

## Design Principles

1. **Boring is Better**: Use proven solutions over novel approaches
2. **Safety First**: Fail safely, never corrupt system
3. **Predictability**: Same input always produces same output
4. **Transparency**: User always knows what will happen
5. **Respect Fedora**: Don't fight the ecosystem
6. **Unix Philosophy**: Do one thing well

## References

- RPM documentation: https://rpm.org/documentation.html
- libdnf5 documentation: https://github.com/rpm-software-management/libdnf
- Fedora Packaging Guidelines: https://docs.fedoraproject.org/en-US/packaging-guidelines/
