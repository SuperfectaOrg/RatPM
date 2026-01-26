#!/bin/bash
set -e

INSTALL_DIR="/usr/bin"
CONFIG_DIR="/etc/ratpm"
SYSTEMD_DIR="/usr/lib/systemd/system"
MAN_DIR="/usr/share/man"
CACHE_DIR="/var/cache/ratpm"
LIB_DIR="/var/lib/ratpm"

if [ "$EUID" -ne 0 ]; then 
    echo "Error: This script must be run as root"
    exit 1
fi

echo "Installing RatPM..."

if [ ! -f "target/release/ratpm" ]; then
    echo "Error: Binary not found. Run 'cargo build --release' first."
    exit 1
fi

echo "Installing binary..."
install -D -m 755 target/release/ratpm "$INSTALL_DIR/ratpm"

echo "Installing configuration..."
mkdir -p "$CONFIG_DIR"
if [ ! -f "$CONFIG_DIR/ratpm.toml" ]; then
    install -D -m 644 ratpm.toml.example "$CONFIG_DIR/ratpm.toml"
    echo "Installed default configuration to $CONFIG_DIR/ratpm.toml"
else
    echo "Configuration already exists at $CONFIG_DIR/ratpm.toml (skipping)"
fi

echo "Installing systemd units..."
install -D -m 644 systemd/ratpm-refresh.service "$SYSTEMD_DIR/ratpm-refresh.service"
install -D -m 644 systemd/ratpm-refresh.timer "$SYSTEMD_DIR/ratpm-refresh.timer"

echo "Installing man pages..."
mkdir -p "$MAN_DIR/man8" "$MAN_DIR/man5"
if command -v pandoc &> /dev/null; then
    pandoc -s -t man docs/ratpm.8.md -o "$MAN_DIR/man8/ratpm.8"
    pandoc -s -t man docs/ratpm.toml.5.md -o "$MAN_DIR/man5/ratpm.toml.5"
    echo "Man pages installed"
else
    echo "Warning: pandoc not found, skipping man page installation"
    echo "Install pandoc and run: make install-man"
fi

echo "Creating directories..."
mkdir -p "$CACHE_DIR" "$LIB_DIR"

echo "Reloading systemd..."
systemctl daemon-reload

echo ""
echo "RatPM installed successfully!"
echo ""
echo "To enable automatic repository refresh:"
echo "  systemctl enable --now ratpm-refresh.timer"
echo ""
echo "To view configuration:"
echo "  cat $CONFIG_DIR/ratpm.toml"
echo ""
echo "To view man pages:"
echo "  man ratpm"
echo "  man ratpm.toml"
