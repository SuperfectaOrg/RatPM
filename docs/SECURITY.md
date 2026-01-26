# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@superfecta.org**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

## What to Include

Please include the following information in your report:

- Type of vulnerability
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Security Considerations

### Privilege Model

RatPM requires root privileges for write operations (install, remove, upgrade). This is enforced at multiple levels:

1. CLI layer checks effective UID
2. Core layer validates privileges before lock acquisition
3. RPM operations fail without proper permissions

### Lock File Security

The lock file (`/var/lock/ratpm.lock`) prevents concurrent operations:

- Created with restrictive permissions (644)
- Contains PID of holder
- Cleaned up on process exit
- Detects and removes stale locks

### Package Verification

By default, RatPM verifies:

1. Repository metadata GPG signatures
2. Package checksums from metadata
3. Package RPM GPG signatures
4. File integrity during installation

Disabling verification is possible but strongly discouraged:
```toml
[repos]
gpgcheck = false  # NOT RECOMMENDED

[transaction]
verify_signatures = false  # NOT RECOMMENDED
```

### Configuration Security

Configuration files should have appropriate permissions:
```bash
# System configuration
sudo chmod 644 /etc/ratpm/ratpm.toml
sudo chown root:root /etc/ratpm/ratpm.toml

# User configuration
chmod 644 ~/.config/ratpm/ratpm.toml
```

### Repository Security

Only use repositories from trusted sources. Repository definitions should:

- Use HTTPS URLs when possible
- Specify GPG keys for verification
- Have `gpgcheck=1` enabled

Example secure repository:
```ini
[fedora]
name=Fedora $releasever - $basearch
baseurl=https://download.fedoraproject.org/pub/fedora/linux/releases/$releasever/Everything/$basearch/os/
enabled=1
gpgcheck=1
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-fedora-$releasever-$basearch
```

### Known Security Considerations

1. **Scriptlet Execution**: Package scriptlets run as root during installation. Future versions will implement sandboxing.

2. **Network Operations**: Package downloads occur over network. Always use HTTPS repositories when available.

3. **Cache Permissions**: Downloaded packages are cached in `/var/cache/ratpm/`. Ensure this directory is protected.

4. **Transaction History**: May contain sensitive information. Protected by file system permissions.

## Security Best Practices

### For Users

1. Only install packages from trusted repositories
2. Keep RatPM updated to the latest version
3. Enable GPG verification (default)
4. Use HTTPS for repository URLs
5. Review transaction summaries before confirming
6. Run `ratpm doctor` periodically to check system health

### For Administrators

1. Restrict access to `/etc/ratpm/ratpm.toml`
2. Monitor `/var/log/` for suspicious activity
3. Audit repository configurations regularly
4. Keep GPG keys up to date
5. Use the `ratpm history` command to audit changes

### For Developers

1. Validate all user input
2. Use Rust's type system to prevent errors
3. Avoid `unsafe` code unless absolutely necessary
4. Never execute shell commands with unsanitized input
5. Follow secure coding guidelines

## Threat Model

### In Scope

- Privilege escalation
- Arbitrary code execution
- Package tampering
- Repository spoofing
- Lock bypass
- Configuration injection

### Out of Scope

- Physical access attacks
- Kernel vulnerabilities
- RPM/libdnf vulnerabilities (report to upstream)
- Social engineering

## Vulnerability Disclosure Timeline

1. **Day 0**: Vulnerability reported
2. **Day 2**: Acknowledgment sent to reporter
3. **Day 7**: Initial assessment completed
4. **Day 30**: Fix developed and tested
5. **Day 45**: Security advisory published
6. **Day 60**: Public disclosure (if fix available)

We follow coordinated disclosure practices and work with reporters to ensure proper credit.

## Security Updates

Security updates are released as patch versions (e.g., 1.0.1) and include:

- Description of the vulnerability
- CVE identifier (if assigned)
- Credit to the reporter
- Mitigation steps
- Patch details

Subscribe to security announcements at: https://github.com/SuperfectaOrg/RatPM/security/advisories

## Contact

- Security Email: security@superfecta.org
- Team Email: team@superfecta.org
- GitHub: https://github.com/SuperfectaOrg/RatPM/security

## PGP Key

Coming soon.
