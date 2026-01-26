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

echo "Uninstalling RatPM..."

echo "Stopping and disabling systemd timer..."
systemctl stop ratpm-refresh.timer 2>/dev/null || true
systemctl disable ratpm-refresh.timer 2>/dev/null || true

echo "Removing systemd units..."
rm -f "$SYSTEMD_DIR/ratpm-refresh.service"
rm -f "$SYSTEMD_DIR/ratpm-refresh.timer"
systemctl daemon-reload

echo "Removing binary..."
rm -f "$INSTALL_DIR/ratpm"

echo "Removing man pages..."
rm -f "$MAN_DIR/man8/ratpm.8"
rm -f "$MAN_DIR/man5/ratpm.toml.5"

read -p "Remove configuration files? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Removing configuration..."
    rm -rf "$CONFIG_DIR"
else
    echo "Keeping configuration at $CONFIG_DIR"
fi

read -p "Remove cache and data? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Removing cache and data..."
    rm -rf "$CACHE_DIR"
    rm -rf "$LIB_DIR"
else
    echo "Keeping cache at $CACHE_DIR"
    echo "Keeping data at $LIB_DIR"
fi

echo ""
echo "RatPM uninstalled successfully!"
