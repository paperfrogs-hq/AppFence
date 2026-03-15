#!/bin/bash
# Installation script for AppFence daemon

set -e

echo "======================================"
echo "AppFence Daemon Installation"
echo "======================================"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "Error: This script must be run as root"
    echo "Please run: sudo $0"
    exit 1
fi

# Detect installation paths
INSTALL_PREFIX="${INSTALL_PREFIX:-/usr}"
BIN_DIR="${INSTALL_PREFIX}/bin"
SYSTEMD_DIR="/usr/lib/systemd/system"
DBUS_SYSTEM_SERVICES="/usr/share/dbus-1/system-services"
DBUS_SYSTEM_CONF="/usr/share/dbus-1/system.d"
POLKIT_DIR="/usr/share/polkit-1/actions"

echo "Installation paths:"
echo "  Binary: ${BIN_DIR}/apfd"
echo "  Systemd: ${SYSTEMD_DIR}/apf-daemon.service"
echo "  DBus: ${DBUS_SYSTEM_SERVICES}/org.apf.Daemon.service"
echo "  Polkit: ${POLKIT_DIR}/org.apf.policy"
echo ""

# Build the daemon in release mode
echo "Building APF daemon..."
cargo build --release --bin apfd

# Install binary
echo "Installing binary..."
install -D -m 755 target/release/apfd "${BIN_DIR}/apfd"

# Install systemd service
echo "Installing systemd service..."
install -D -m 644 systemd/apf-daemon.service "${SYSTEMD_DIR}/apf-daemon.service"

# Install DBus service file
echo "Installing DBus service file..."
install -D -m 644 dbus/org.apf.Daemon.service "${DBUS_SYSTEM_SERVICES}/org.apf.Daemon.service"

# Install DBus configuration
echo "Installing DBus configuration..."
install -D -m 644 dbus/org.apf.Daemon.conf "${DBUS_SYSTEM_CONF}/org.apf.Daemon.conf"

# Install Polkit policy
echo "Installing Polkit policy..."
install -D -m 644 polkit/org.apf.policy "${POLKIT_DIR}/org.apf.policy"

# Create data directories
echo "Creating data directories..."
mkdir -p /var/lib/apf
chmod 700 /var/lib/apf

mkdir -p /etc/appfence
chmod 755 /etc/appfence

mkdir -p /var/log/apf
chmod 700 /var/log/apf

# Reload systemd
echo "Reloading systemd..."
systemctl daemon-reload

# Reload DBus
echo "Reloading DBus configuration..."
if systemctl is-active --quiet dbus; then
    systemctl reload dbus || true
fi

echo ""
echo "======================================"
echo "Installation Complete!"
echo "======================================"
echo ""
echo "To start the daemon:"
echo "  sudo systemctl start apf-daemon"
echo ""
echo "To enable at boot:"
echo "  sudo systemctl enable apf-daemon"
echo ""
echo "To check status:"
echo "  sudo systemctl status apf-daemon"
echo ""
echo "To view logs:"
echo "  sudo journalctl -u apf-daemon -f"
echo ""
