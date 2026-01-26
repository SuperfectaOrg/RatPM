#!/bin/bash
set -e

echo "Setting up RatPM development environment..."

if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed"
    echo "Install from: https://rustup.rs/"
    exit 1
fi

RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo "Rust version: $RUST_VERSION"

if command -v dnf &> /dev/null; then
    echo "Detected Fedora/RHEL system"
    echo "Installing development dependencies..."
    
    sudo dnf install -y \
        gcc \
        rpm-devel \
        make \
        git \
        pandoc \
        clippy \
        rustfmt
    
    echo "Installing libdnf5 (if available)..."
    sudo dnf install -y libdnf5-devel || echo "Note: libdnf5-devel not available"
    
elif command -v apt &> /dev/null; then
    echo "Detected Debian/Ubuntu system"
    echo "Warning: RatPM is designed for Fedora-based systems"
    echo "Some features may not work correctly"
    
    sudo apt install -y \
        gcc \
        librpm-dev \
        make \
        git \
        pandoc
else
    echo "Warning: Unknown package manager"
    echo "Please manually install: gcc, rpm-devel, make, git, pandoc"
fi

echo "Installing Rust tools..."
rustup component add rustfmt clippy

echo "Creating development directories..."
mkdir -p target
mkdir -p .cache

if [ ! -f ".git/hooks/pre-commit" ]; then
    echo "Installing git pre-commit hook..."
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
set -e

echo "Running pre-commit checks..."

echo "Checking format..."
cargo fmt --all -- --check

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running tests..."
cargo test

echo "All checks passed!"
EOF
    chmod +x .git/hooks/pre-commit
    echo "Git pre-commit hook installed"
fi

echo ""
echo "Development environment setup complete!"
echo ""
echo "Quick start:"
echo "  cargo build          # Build debug binary"
echo "  cargo test           # Run tests"
echo "  cargo run -- --help  # Run RatPM with help"
echo "  RUST_LOG=debug cargo run -- search vim  # Run with debug logging"
echo ""
echo "Before committing:"
echo "  cargo fmt            # Format code"
echo "  cargo clippy         # Lint code"
echo "  cargo test           # Run tests"
