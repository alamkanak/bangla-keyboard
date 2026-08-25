#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
APP_DIR="$SCRIPT_DIR/../.."
PLATFORM_DIR="$SCRIPT_DIR"
BUILD_DIR="$APP_DIR/target/macos-ime"
APP_BUNDLE="$BUILD_DIR/BanglaKeyboard.app"
INSTALL_DIR="$HOME/Library/Input Methods"

echo "==> Building Bangla Keyboard IME..."

# Build Rust engine first
echo "==> Building Rust engine..."
cd "$APP_DIR"
if command -v mise &>/dev/null; then
    mise exec -- cargo build -p engine-ffi --release
else
    cargo build -p engine-ffi --release
fi
cd "$PLATFORM_DIR"

# Clean previous build
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Create .app bundle structure
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources/data"

# Copy Info.plist
cp "$PLATFORM_DIR/Info.plist" "$APP_BUNDLE/Contents/Info.plist"

# Copy icons
cp "$PLATFORM_DIR/BanglaKeyboard/AppIcon.tiff" "$APP_BUNDLE/Contents/Resources/"
cp "$PLATFORM_DIR/BanglaKeyboard/AppIcon.png" "$APP_BUNDLE/Contents/Resources/" 2>/dev/null || true

# Copy localization strings (for display name)
for lproj in "$PLATFORM_DIR/BanglaKeyboard/"*.lproj; do
    cp -R "$lproj" "$APP_BUNDLE/Contents/Resources/"
done

# Copy data files for the engine
cp -R "$APP_DIR/data/"* "$APP_BUNDLE/Contents/Resources/data/"

# Write PkgInfo
echo -n "APPL????" > "$APP_BUNDLE/Contents/PkgInfo"

# Compile Swift sources with Rust FFI linking
echo "==> Compiling Swift..."
SWIFT_FILES=(
    "$PLATFORM_DIR/BanglaKeyboard/main.swift"
    "$PLATFORM_DIR/BanglaKeyboard/AppDelegate.swift"
    "$PLATFORM_DIR/BanglaKeyboard/BanglaKeyboardController.swift"
)

RUST_LIB="$APP_DIR/target/release/libengine_ffi.a"
FFI_HEADER_DIR="$APP_DIR/crates/engine-ffi"

swiftc \
    -module-name BanglaKeyboard \
    -target arm64-apple-macos13.0 \
    -sdk "$(xcrun --show-sdk-path)" \
    -framework Cocoa \
    -framework InputMethodKit \
    -import-objc-header "$FFI_HEADER_DIR/bangla_keyboard_engine.h" \
    -L "$APP_DIR/target/release" \
    -lengine_ffi \
    -o "$APP_BUNDLE/Contents/MacOS/BanglaKeyboard" \
    "${SWIFT_FILES[@]}"

# Sign with ad-hoc signature (required for IMKit)
echo "==> Signing..."
codesign --force --sign - \
    --entitlements "$PLATFORM_DIR/BanglaKeyboard/BanglaKeyboard.entitlements" \
    "$APP_BUNDLE"

echo "==> Build complete: $APP_BUNDLE"

# Build Settings app (Tauri) and embed it
SETTINGS_DIR="$APP_DIR/crates/tauri-settings"
FRONTEND_DIR="$SETTINGS_DIR/src-frontend"

if [ -d "$FRONTEND_DIR/node_modules" ]; then
    echo "==> Building Settings app..."
    cd "$FRONTEND_DIR"
    npx vite build
    cd "$SETTINGS_DIR"

    if command -v mise &>/dev/null; then
        mise exec -- cargo tauri build --no-bundle
    else
        cargo tauri build --no-bundle
    fi

    SETTINGS_BIN="$APP_DIR/target/release/tauri-settings"

    if [ -n "$SETTINGS_BIN" ] && [ -f "$SETTINGS_BIN" ]; then
        SETTINGS_APP="$APP_BUNDLE/Contents/Resources/BanglaKeyboardSettings.app"
        mkdir -p "$SETTINGS_APP/Contents/MacOS"
        mkdir -p "$SETTINGS_APP/Contents/Resources"
        cp "$SETTINGS_BIN" "$SETTINGS_APP/Contents/MacOS/BanglaKeyboardSettings"
        # Minimal Info.plist for the settings app
        cat > "$SETTINGS_APP/Contents/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>dev.banglakeyboard.settings</string>
    <key>CFBundleName</key>
    <string>Bangla Keyboard Settings</string>
    <key>CFBundleExecutable</key>
    <string>BanglaKeyboardSettings</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
</dict>
</plist>
PLIST
        echo "==> Settings app embedded"
    else
        echo "==> Settings app build skipped (tauri build failed or not available)"
    fi
else
    echo "==> Settings app skipped (run 'cd crates/tauri-settings/src-frontend && npm install' first)"
fi

# Re-sign after embedding
codesign --force --sign - \
    --entitlements "$PLATFORM_DIR/BanglaKeyboard/BanglaKeyboard.entitlements" \
    "$APP_BUNDLE" 2>/dev/null || true

# Install to ~/Library/Input Methods/
echo "==> Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

# Kill existing instance if running
pkill -f "BanglaKeyboard.app" 2>/dev/null || true
sleep 1

# Remove old installation
rm -rf "$INSTALL_DIR/BanglaKeyboard.app"

# Copy new build
cp -R "$APP_BUNDLE" "$INSTALL_DIR/"

echo "==> Installed!"
echo ""
echo "Next steps:"
echo "  1. Open System Settings → Keyboard → Input Sources"
echo "  2. Click 'Edit' next to Input Sources"
echo "  3. Click '+', look under 'Bangla' for 'Bangla Keyboard'"
echo "  4. Add it, then use Ctrl+Space or globe key to switch"
echo ""
echo "If it doesn't appear, try logging out and back in."
