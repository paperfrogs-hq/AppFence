#!/bin/bash
# Development environment setup script

set -e

echo "🚀 Setting up AppFence development environment..."

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust found: $(rustc --version)"
fi

# Install development tools
echo "📦 Installing development tools..."
cargo install cargo-watch cargo-audit cargo-deny || true

# Check for system dependencies
echo "🔍 Checking system dependencies..."

check_dependency() {
    if command -v "$1" &> /dev/null; then
        echo "  ✅ $1 found"
        return 0
    else
        echo "  ❌ $1 not found"
        return 1
    fi
}

MISSING_DEPS=0

check_dependency "systemctl" || MISSING_DEPS=1
check_dependency "bwrap" || MISSING_DEPS=1

if [ $MISSING_DEPS -eq 1 ]; then
    echo ""
    echo "⚠️  Some dependencies are missing. Install them based on your distro:"
    echo ""
    echo "Fedora:"
    echo "  sudo dnf install systemd-devel sqlite-devel bubblewrap"
    echo ""
    echo "Ubuntu:"
    echo "  sudo apt install libsystemd-dev libsqlite3-dev bubblewrap"
    echo ""
    echo "Arch:"
    echo "  sudo pacman -S systemd sqlite bubblewrap"
fi

# Build project
echo ""
echo "🔨 Building project..."
cargo build

echo ""
echo "✅ Development environment setup complete!"
echo ""
echo "Next steps:"
echo "  cargo test --all         # Run tests"
echo "  cargo run -p apf-daemon  # Run daemon"
echo "  cargo watch -x check     # Auto-check on file changes"
