#!/bin/bash
set -e

# Builds BanglaKeyboard.app to target/macos-ime/
# Does NOT install or register anything. Used by both dev and prod scripts.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
APP_DIR="$SCRIPT_DIR/../.."
PLATFORM_DIR="$SCRIPT_DIR"
BUILD_DIR="$APP_DIR/target/macos-ime"
APP_BUNDLE="$BUILD_DIR/BanglaKeyboard.app"

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
    "$PLATFORM_DIR/BanglaKeyboard/InputSourceRegistrar.swift"
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
    "$RUST_LIB" \
    -o "$APP_BUNDLE/Contents/MacOS/BanglaKeyboard" \
    "${SWIFT_FILES[@]}"

# Build the lightweight registration CLI tool (no InputMethodKit dependency)
echo "==> Compiling registration helper..."
swiftc \
    -module-name BanglaKeyboardRegister \
    -target arm64-apple-macos13.0 \
    -sdk "$(xcrun --show-sdk-path)" \
    -framework Carbon \
    -o "$APP_BUNDLE/Contents/MacOS/bangla-keyboard-register" \
    "$PLATFORM_DIR/BanglaKeyboard/register-cli.swift" \
    "$PLATFORM_DIR/BanglaKeyboard/InputSourceRegistrar.swift"

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
        mise exec -- cargo tauri build
    else
        cargo tauri build
    fi

    # Find the Tauri-built .app bundle (includes frontend assets in Resources/)
    TAURI_APP=$(find "$APP_DIR/target/release/bundle" -name "*.app" -maxdepth 3 2>/dev/null | head -1)

    if [ -n "$TAURI_APP" ] && [ -d "$TAURI_APP" ]; then
        SETTINGS_APP="$APP_BUNDLE/Contents/Resources/BanglaKeyboardSettings.app"
        rm -rf "$SETTINGS_APP"
        cp -R "$TAURI_APP" "$SETTINGS_APP"
        /usr/libexec/PlistBuddy -c "Set :CFBundleIdentifier dev.banglakeyboard.settings" \
            "$SETTINGS_APP/Contents/Info.plist" 2>/dev/null || true
        echo "==> Settings app embedded from $TAURI_APP"
    else
        echo "==> Settings app build skipped (tauri bundle not found)"
    fi
else
    echo "==> Settings app skipped (run 'cd crates/tauri-settings/src-frontend && npm install' first)"
fi

# Re-sign after embedding
codesign --force --sign - \
    --entitlements "$PLATFORM_DIR/BanglaKeyboard/BanglaKeyboard.entitlements" \
    "$APP_BUNDLE" 2>/dev/null || true

echo "==> .app bundle ready at $APP_BUNDLE"
