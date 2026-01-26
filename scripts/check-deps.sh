#!/bin/bash

echo "Checking RatPM dependencies..."
echo ""

MISSING_DEPS=0

check_command() {
    if command -v "$1" &> /dev/null; then
        VERSION=$($2 2>&1 || echo "unknown")
        echo "✓ $1: $VERSION"
        return 0
    else
        echo "✗ $1: NOT FOUND"
        return 1
    fi
}

check_library() {
    if pkg-config --exists "$1" 2>/dev/null; then
        VERSION=$(pkg-config --modversion "$1" 2>/dev/null || echo "unknown")
        echo "✓ $1: $VERSION"
        return 0
    else
        echo "✗ $1: NOT FOUND"
        return 1
    fi
}

echo "Build Tools:"
check_command rustc "rustc --version" || MISSING_DEPS=1
check_command cargo "cargo --version" || MISSING_DEPS=1
check_command gcc "gcc --version | head -n1" || MISSING_DEPS=1
check_command make "make --version | head -n1" || MISSING_DEPS=1

echo ""
echo "Rust Tools:"
check_command rustfmt "rustfmt --version" || MISSING_DEPS=1
check_command clippy-driver "cargo clippy --version" || MISSING_DEPS=1

echo ""
echo "System Libraries:"
check_library rpm || echo "  (rpm-devel package required)"
echo ""

echo "Optional Tools:"
check_command pandoc "pandoc --version | head -n1" || echo "  (needed for man page generation)"
check_command rpmbuild "rpmbuild --version | head -n1" || echo "  (needed for RPM packaging)"
check_command systemctl "systemctl --version | head -n1" || echo "  (needed for systemd integration)"

echo ""
echo "Git Configuration:"
if [ -d ".git" ]; then
    echo "✓ Git repository initialized"
    if [ -f ".git/hooks/pre-commit" ]; then
        echo "✓ Pre-commit hook installed"
    else
        echo "  (run scripts/dev-setup.sh to install pre-commit hook)"
    fi
else
    echo "✗ Not a git repository"
fi

echo ""
echo "Cargo Configuration:"
if [ -f "Cargo.toml" ]; then
    echo "✓ Cargo.toml present"
    VERSION=$(grep "^version" Cargo.toml | head -n1 | cut -d'"' -f2)
    echo "  Version: $VERSION"
else
    echo "✗ Cargo.toml not found"
    MISSING_DEPS=1
fi

echo ""
if [ $MISSING_DEPS -eq 0 ]; then
    echo "All required dependencies are installed!"
    exit 0
else
    echo "Some dependencies are missing."
    echo ""
    echo "On Fedora/RHEL, install with:"
    echo "  sudo dnf install rust cargo gcc rpm-devel make git pandoc"
    echo ""
    echo "Then run: scripts/dev-setup.sh"
    exit 1
fi
