% RATPM(8) RatPM 1.0.0
% RatOS Team
% January 2026

# NAME

ratpm - RatOS Package Manager

# SYNOPSIS

**ratpm** [*OPTIONS*] *COMMAND* [*ARGS*]

# DESCRIPTION

RatPM is the primary package management frontend for RatOS, a Fedora-based Linux distribution. It provides a policy enforcement layer and user interface on top of RPM and libdnf5.

# COMMANDS

**install** *PACKAGES*...
: Install one or more packages and their dependencies.

**remove** *PACKAGES*...
: Remove one or more packages from the system.

**update**
: Update repository metadata from all enabled repositories.

**upgrade** [*PACKAGES*...]
: Upgrade all installed packages or specific packages to their latest versions.

**search** *QUERY*
: Search for packages matching the given query string.

**info** *PACKAGE*
: Display detailed information about a package.

**list** [**--installed**|**--available**]
: List packages. Without options, lists all packages. Use **--installed** to list only installed packages or **--available** to list only available packages.

**sync**
: Synchronize package databases and verify integrity.

**doctor**
: Run system diagnostics and check for common issues.

**history** [**--limit** *N*]
: Display transaction history. Optionally limit to the most recent *N* entries.

# OPTIONS

**-y**, **--assume-yes**
: Automatically answer yes to all prompts. Use with caution.

**--no-color**
: Disable colored output.

**-h**, **--help**
: Print help information.

**-V**, **--version**
: Print version information.

# CONFIGURATION

System configuration: */etc/ratpm/ratpm.toml*

User configuration: *~/.config/ratpm/ratpm.toml*

See **ratpm.toml**(5) for configuration file format.

# FILES

*/etc/ratpm/ratpm.toml*
: System-wide configuration file.

*/var/cache/ratpm/*
: Package cache directory.

*/var/lock/ratpm.lock*
: Lock file to prevent concurrent operations.

*/etc/yum.repos.d/*
: Repository definition files.

*/var/lib/rpm/*
: RPM database.

# EXIT STATUS

**0**
: Success

**1**
: General error or package not found

**2**
: Dependency conflict

**3**
: Transaction check failed

**4**
: Transaction execution failed

**5**
: Network error

**6**
: Repository error

**7**
: Insufficient disk space

**8**
: Configuration error

**9**
: RPM database error

**10**
: Scriptlet execution failed

**13**
: Permission denied

**14**
: Lock held by another process

# EXAMPLES

Install a package:
```
# ratpm install neovim
```

Remove a package:
```
# ratpm remove vim
```

Search for packages:
```
$ ratpm search editor
```

Upgrade all packages:
```
# ratpm upgrade
```

Update repository metadata:
```
# ratpm update
```

Check system health:
```
$ ratpm doctor
```

# SECURITY

RatPM requires root privileges for all write operations (install, remove, upgrade, update). It uses a system-wide lock to prevent concurrent package operations.

GPG verification is enabled by default for both repository metadata and packages. This can be configured in the configuration file.

# BUGS

Report bugs at: https://github.com/SuperfectaOrg/RatPM/issues

# SEE ALSO

**rpm**(8), **dnf**(8), **yum**(8), **ratpm.toml**(5)

# COPYRIGHT

Copyright © 2026 RatOS Team. License GPLv3+: GNU GPL version 3 or later.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
