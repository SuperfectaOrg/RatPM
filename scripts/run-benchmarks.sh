#!/bin/bash
set -e

echo "Running RatPM benchmarks..."
echo ""

if [ ! -d "benches" ]; then
    echo "No benchmarks directory found."
    echo "Creating benchmark structure..."
    mkdir -p benches
fi

echo "Building optimized binary..."
cargo build --release

RATPM_BIN="./target/release/ratpm"

if [ ! -f "$RATPM_BIN" ]; then
    echo "Error: Binary not found at $RATPM_BIN"
    exit 1
fi

echo ""
echo "=== Benchmark: Version Check ==="
time for i in {1..100}; do
    $RATPM_BIN --version > /dev/null
done

echo ""
echo "=== Benchmark: Help Display ==="
time for i in {1..100}; do
    $RATPM_BIN --help > /dev/null
done

echo ""
echo "=== Benchmark: Search (No Root) ==="
time for i in {1..10}; do
    $RATPM_BIN search vim > /dev/null 2>&1 || true
done

echo ""
echo "=== Benchmark: Info (No Root) ==="
time for i in {1..10}; do
    $RATPM_BIN info bash > /dev/null 2>&1 || true
done

echo ""
echo "=== Benchmark: List Installed (No Root) ==="
time for i in {1..10}; do
    $RATPM_BIN list --installed > /dev/null 2>&1 || true
done

echo ""
echo "=== Binary Size ==="
ls -lh "$RATPM_BIN" | awk '{print "Size: " $5}'

echo ""
echo "=== Memory Usage (Idle) ==="
/usr/bin/time -v $RATPM_BIN --version 2>&1 | grep "Maximum resident set size" || echo "N/A"

echo ""
echo "Benchmarks complete!"
