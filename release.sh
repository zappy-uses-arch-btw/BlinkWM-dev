#!/bin/bash
# BlinkWM Release Build Script

set -e

VERSION="${1:-0.1.0}"
RELEASE_DIR="release/$VERSION"

echo "Building BlinkWM v$VERSION..."

# Build release binaries
echo "Building release binaries..."
cargo build --release

# Create release directory
mkdir -p "$RELEASE_DIR"

# Copy binaries
echo "Copying binaries..."
cp target/release/blink-wm "$RELEASE_DIR/"
cp target/release/blink "$RELEASE_DIR/"
cp target/release/blink-launch "$RELEASE_DIR/"
cp target/release/blink-conf "$RELEASE_DIR/"
cp target/release/blink-bar "$RELEASE_DIR/"
cp target/release/blink-pkg "$RELEASE_DIR/"

# Copy config
mkdir -p "$RELEASE_DIR/colorschemes"
cp config/config.lua "$RELEASE_DIR/"
cp colorschemes/*.lua "$RELEASE_DIR/colorschemes/"

# Copy PKGBUILD
mkdir -p "$RELEASE_DIR/pkg"
cp pkg/PKGBUILD "$RELEASE_DIR/pkg/"
cp pkg/.SRCINFO "$RELEASE_DIR/pkg/"
rm -rf "$RELEASE_DIR/pkg/files"
cp -r pkg/files "$RELEASE_DIR/pkg/"

echo "Release v$VERSION ready in $RELEASE_DIR/"
echo ""
echo "Contents:"
ls -lh "$RELEASE_DIR"
