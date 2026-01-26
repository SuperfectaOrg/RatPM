#!/bin/bash
set -e

TEMP_ROOT=$(mktemp -d)
trap "rm -rf $TEMP_ROOT" EXIT

echo "Testing RatPM installation in temporary directory: $TEMP_ROOT"

export INSTALL_DIR="$TEMP_ROOT/usr/bin"
export CONFIG_DIR="$TEMP_ROOT/etc/ratpm"
export SYSTEMD_DIR="$TEMP_ROOT/usr/lib/systemd/system"
export MAN_DIR="$TEMP_ROOT/usr/share/man"
export CACHE_DIR="$TEMP_ROOT/var/cache/ratpm"
export LIB_DIR="$TEMP_ROOT/var/lib/ratpm"

echo "Building release binary..."
cargo build --release

echo "Creating directory structure..."
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"
mkdir -p "$SYSTEMD_DIR"
mkdir -p "$MAN_DIR/man8"
mkdir -p "$MAN_DIR/man5"
mkdir -p "$CACHE_DIR"
mkdir -p "$LIB_DIR"

echo "Installing binary..."
install -D -m 755 target/release/ratpm "$INSTALL_DIR/ratpm"

echo "Installing configuration..."
install -D -m 644 ratpm.toml.example "$CONFIG_DIR/ratpm.toml"

echo "Installing systemd units..."
install -D -m 644 systemd/ratpm-refresh.service "$SYSTEMD_DIR/ratpm-refresh.service"
install -D -m 644 systemd/ratpm-refresh.timer "$SYSTEMD_DIR/ratpm-refresh.timer"

echo "Verifying installation..."

if [ ! -f "$INSTALL_DIR/ratpm" ]; then
    echo "Error: Binary not installed"
    exit 1
fi

if [ ! -x "$INSTALL_DIR/ratpm" ]; then
    echo "Error: Binary not executable"
    exit 1
fi

if [ ! -f "$CONFIG_DIR/ratpm.toml" ]; then
    echo "Error: Configuration not installed"
    exit 1
fi

if [ ! -f "$SYSTEMD_DIR/ratpm-refresh.service" ]; then
    echo "Error: systemd service not installed"
    exit 1
fi

if [ ! -f "$SYSTEMD_DIR/ratpm-refresh.timer" ]; then
    echo "Error: systemd timer not installed"
    exit 1
fi

echo "Testing binary execution..."
"$INSTALL_DIR/ratpm" --version

echo "Validating configuration..."
if ! grep -q "backend = \"fedora\"" "$CONFIG_DIR/ratpm.toml"; then
    echo "Error: Configuration validation failed"
    exit 1
fi

echo "Testing help output..."
"$INSTALL_DIR/ratpm" --help > /dev/null

echo "Checking directory permissions..."
if [ ! -d "$CACHE_DIR" ]; then
    echo "Error: Cache directory not created"
    exit 1
fi

if [ ! -d "$LIB_DIR" ]; then
    echo "Error: Lib directory not created"
    exit 1
fi

echo ""
echo "Installation test PASSED!"
echo ""
echo "Installed files:"
find "$TEMP_ROOT" -type f | sed "s|$TEMP_ROOT||" | sort
echo ""
echo "Directory structure:"
tree -L 3 "$TEMP_ROOT" 2>/dev/null || find "$TEMP_ROOT" -type d | sed "s|$TEMP_ROOT||" | sort
