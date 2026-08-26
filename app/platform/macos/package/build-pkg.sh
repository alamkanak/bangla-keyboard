#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PLATFORM_DIR="$SCRIPT_DIR/.."
APP_DIR="$PLATFORM_DIR/../.."
BUILD_DIR="$APP_DIR/target/macos-ime"
APP_BUNDLE="$BUILD_DIR/BanglaKeyboard.app"
PKG_DIR="$BUILD_DIR/pkg"
PKG_OUTPUT="$BUILD_DIR/BanglaKeyboard.pkg"
VERSION="${VERSION:-0.1.0}"
IDENTIFIER="dev.banglakeyboard.inputmethod.BanglaKeyboard"

echo "==> Creating .pkg installer..."

# Always rebuild to ensure the pkg has the latest code
"$PLATFORM_DIR/build.sh"

# Clean previous pkg artifacts
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR/root"
mkdir -p "$PKG_DIR/scripts"

# Bundle the .app inside the scripts dir so postinstall can copy it to ~/Library/Input Methods
cp -R "$APP_BUNDLE" "$PKG_DIR/scripts/BanglaKeyboard.app"

# Copy postinstall script
cp "$SCRIPT_DIR/postinstall" "$PKG_DIR/scripts/postinstall"
chmod +x "$PKG_DIR/scripts/postinstall"

# Build component package with empty payload (all work done in postinstall)
pkgbuild \
    --nopayload \
    --scripts "$PKG_DIR/scripts" \
    --identifier "$IDENTIFIER" \
    --version "$VERSION" \
    "$PKG_DIR/BanglaKeyboard-component.pkg"

# Copy Distribution XML and conclusion page
cp "$SCRIPT_DIR/Distribution" "$PKG_DIR/Distribution"
cp "$SCRIPT_DIR/conclusion.html" "$PKG_DIR/conclusion.html"

# Build the product archive with distribution (includes conclusion page)
productbuild \
    --distribution "$PKG_DIR/Distribution" \
    --package-path "$PKG_DIR" \
    --resources "$PKG_DIR" \
    "$PKG_OUTPUT"

echo "==> Package created: $PKG_OUTPUT"
