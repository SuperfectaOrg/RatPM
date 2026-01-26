# Frequently Asked Questions

## General Questions

### What is RatPM?

RatPM is the primary package manager for RatOS, a Fedora-based Linux distribution. It provides a modern, safe, and user-friendly interface for managing RPM packages while using proven Fedora infrastructure (RPM and libdnf5) under the hood.

### How is RatPM different from DNF?

RatPM is not a replacement for DNF's core functionality—it uses libdnf5 for dependency resolution. The differences are:

- **Policy enforcement**: RatPM adds RatOS-specific validation and policies
- **User experience**: Cleaner output, simpler commands
- **Safety**: Stricter transaction semantics and better error handling
- **Future features**: Will integrate with RatOS-specific features like snapshots and rollback

### Can I use RatPM on standard Fedora?

Yes! RatPM is designed for RatOS but works on any Fedora-based system. It uses standard RPM and repository formats.

### Is RatPM stable for production use?

RatPM v1.0.0 is the initial release with core functionality. While thoroughly tested, it has some limitations (see README.md). For production systems, consider:

- Testing in a VM or container first
- Keeping DNF installed as a fallback
- Reviewing the known limitations

## Installation & Setup

### How do I install RatPM?
```bash
# From source
git clone https://github.com/SuperfectaOrg/RatPM.git
cd RatPM
cargo build --release
sudo ./scripts/install.sh

# From RPM (when available)
sudo dnf install ratpm-1.0.0-1.fc39.x86_64.rpm
```

### Where is the configuration file?

System configuration: `/etc/ratpm/ratpm.toml`
User configuration: `~/.config/ratpm/ratpm.toml`

### How do I enable automatic repository updates?
```bash
sudo systemctl enable --now ratpm-refresh.timer
```

This runs `ratpm update` daily at 6:00 AM.

## Usage Questions

### Why do I need root privileges?

Root is required for operations that modify the system:
- `install` - Writes to system directories
- `remove` - Removes system files
- `upgrade` - Modifies system packages
- `update` - Updates repository metadata

Read-only operations (`search`, `info`, `list`) don't require root.

### How do I install a package?
```bash
sudo ratpm install package-name
```

### How do I search for packages?
```bash
ratpm search keyword
```

No root required for searching.

### Can I install multiple packages at once?

Yes:
```bash
sudo ratpm install vim neovim emacs
```

### How do I upgrade all packages?
```bash
sudo ratpm upgrade
```

Or upgrade specific packages:
```bash
sudo ratpm upgrade vim neovim
```

### How do I remove a package?
```bash
sudo ratpm remove package-name
```

### What does "ratpm sync" do?

`ratpm sync` synchronizes and verifies package databases. Use it if you suspect database corruption or after manually modifying repository files.

### What is "ratpm doctor"?

`ratpm doctor` runs system diagnostics to check for:
- RPM database integrity
- Repository availability
- Configuration errors
- Common issues

## Troubleshooting

### "Permission denied" error

Most RatPM operations require root privileges. Run with `sudo`:
```bash
sudo ratpm install vim
```

### "Lock held by another process"

Another package manager (RatPM, DNF, or RPM) is currently running. Wait for it to finish or find the process:
```bash
# Find process holding lock
lsof /var/lock/ratpm.lock

# If it's a stale lock (process doesn't exist)
sudo rm /var/lock/ratpm.lock
```

### "Package not found"

The package doesn't exist in any enabled repository. Try:

1. Update repository metadata: `sudo ratpm update`
2. Search for similar packages: `ratpm search keyword`
3. Check if the repository is enabled in `/etc/yum.repos.d/`

### "Dependency conflict"

The package you're trying to install conflicts with installed packages. The error message will specify the conflict. You may need to:

1. Remove conflicting packages first
2. Use a different version
3. Wait for repository updates

### "Repository unavailable"

RatPM can't reach a repository. Check:

1. Network connectivity: `ping download.fedoraproject.org`
2. Repository URL in `/etc/yum.repos.d/`
3. Firewall settings

### Transaction failed

If a transaction fails:

1. Check disk space: `df -h`
2. Verify RPM database: `sudo ratpm doctor`
3. Check logs: `journalctl -xe`
4. Try again with debug logging: `RUST_LOG=debug sudo ratpm install package`

### How do I see detailed logs?
```bash
RUST_LOG=debug ratpm command
RUST_LOG=trace ratpm command  # Very verbose
```

## Configuration

### How do I disable colored output?

Command line:
```bash
ratpm --no-color command
```

Configuration file:
```toml
[system]
color = false
```

### How do I change the cache directory?

Edit `/etc/ratpm/ratpm.toml`:
```toml
[system]
cache_dir = "/path/to/cache"
```

### How do I disable GPG verification?

**Not recommended**, but edit `/etc/ratpm/ratpm.toml`:
```toml
[repos]
gpgcheck = false

[transaction]
verify_signatures = false
```

### How often is repository metadata refreshed?

Default: every 24 hours (86400 seconds)

Change in `/etc/ratpm/ratpm.toml`:
```toml
[repos]
metadata_expire = 43200  # 12 hours
```

## Compatibility

### Can I use RatPM alongside DNF?

Yes! They share the same RPM database and repository definitions. However:

- Don't run both simultaneously (lock prevents this)
- Operations from one are visible to the other
- History is tracked separately

### Do DNF repositories work with RatPM?

Yes! RatPM reads the same `.repo` files in `/etc/yum.repos.d/`.

### Can I use DNF plugins with RatPM?

No. RatPM doesn't support DNF plugins. Future versions may add a plugin system.

### Does RatPM work with Flatpak/Snap?

RatPM only manages RPM packages. Use Flatpak/Snap tools for those package formats.

## Advanced Usage

### How do I see transaction history?
```bash
ratpm history

# Limit to last 10 transactions
ratpm history --limit 10
```

### Can I automate installations?

Yes, use `--assume-yes` to skip confirmations:
```bash
sudo ratpm install --assume-yes package-name
```

Or set in configuration:
```toml
[system]
assume_yes = true
```

**Warning**: Use with caution in production.

### How do I add a new repository?

Create a `.repo` file in `/etc/yum.repos.d/`:
```ini
[myrepo]
name=My Repository
baseurl=https://example.com/repo/
enabled=1
gpgcheck=1
gpgkey=https://example.com/RPM-GPG-KEY
```

Then refresh: `sudo ratpm update`

### Can I build RatPM from source?

Yes:
```bash
git clone https://github.com/SuperfectaOrg/RatPM.git
cd RatPM
cargo build --release
```

Binary will be at `target/release/ratpm`.

### How do I contribute?

See CONTRIBUTING.md for guidelines. Contributions are welcome!

## Performance

### Why is the first search slow?

RatPM loads repository metadata on first use. Subsequent searches are faster. Run `sudo ratpm update` to pre-cache metadata.

### How much disk space does RatPM use?

- Binary: ~5-10 MB
- Cache: Varies (100 MB - 2 GB depending on repositories)
- Set `keep_cache = false` to auto-delete downloaded packages

### Can I clear the cache?
```bash
sudo rm -rf /var/cache/ratpm/packages/*
```

Or disable caching:
```toml
[transaction]
keep_cache = false
```

## Security

### Is RatPM secure?

RatPM follows security best practices:
- GPG verification enabled by default
- Requires root for write operations
- Lock prevents concurrent operations
- No shell command injection

See SECURITY.md for details.

### How do I report a security vulnerability?

Email: security@superfecta.org

**Do not** open public GitHub issues for security problems.

### Does RatPM sandbox scriptlets?

Not in v1.0.0. This is planned for future releases.

## Getting Help

### Where can I get help?

1. This FAQ
2. Man pages: `man ratpm`, `man ratpm.toml`
3. GitHub Discussions: https://github.com/SuperfectaOrg/RatPM/discussions
4. GitHub Issues: https://github.com/SuperfectaOrg/RatPM/issues
5. Email: team@superfecta.org

### How do I report a bug?

Open an issue: https://github.com/SuperfectaOrg/RatPM/issues/new

Include:
- RatPM version (`ratpm --version`)
- Operating system
- Full command output
- Steps to reproduce

### Is there a community forum?

Yes! GitHub Discussions: https://github.com/SuperfectaOrg/RatPM/discussions

## Roadmap

### What features are planned?

See GitHub Issues with the `enhancement` label:
https://github.com/SuperfectaOrg/RatPM/labels/enhancement

Major planned features:
- Full libdnf5 integration
- Transaction rollback
- System snapshots
- GUI interface
- Plugin system

### When is the next release?

Follow the project on GitHub for release announcements.

### Can I request a feature?

Yes! Open a feature request: https://github.com/SuperfectaOrg/RatPM/issues/new

Choose "Feature request" template.
