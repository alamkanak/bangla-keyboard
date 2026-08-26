#!/bin/bash
set -e

# Dev script: builds the .app and installs it locally for testing.
# For production, use build.sh + package/build-pkg.sh instead.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
APP_DIR="$SCRIPT_DIR/../.."
BUILD_DIR="$APP_DIR/target/macos-ime"
APP_BUNDLE="$BUILD_DIR/BanglaKeyboard.app"
INSTALL_DIR="$HOME/Library/Input Methods"

# Build the .app
"$SCRIPT_DIR/build.sh"

# Install to ~/Library/Input Methods/
echo "==> Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

pkill -f "BanglaKeyboard.app" 2>/dev/null || true
sleep 1

rm -rf "$INSTALL_DIR/BanglaKeyboard.app"
cp -R "$APP_BUNDLE" "$INSTALL_DIR/"

echo "==> Installed!"
echo ""
echo "Next steps:"
echo "  1. Add input source: System Settings → Keyboard → Input Sources → Edit → + → Bangla → Bangla Keyboard"
echo "  2. Switch to it: Globe key or Ctrl+Space"
echo "  3. Open Settings from the menu bar icon"
