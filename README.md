# Bangla Keyboard

A macOS + Windows input method supporting Phonetic, UniBijoy, and National (Jatiya) keyboard layouts for typing in Bangla.

```
bangla-keyboard/
├── app/
│   ├── crates/
│   │   ├── engine-core/       # Transliteration, key maps, dictionary
│   │   ├── engine-ffi/        # C FFI bindings (cbindgen)
│   │   └── tauri-settings/    # Settings UI (Tauri + Svelte)
│   ├── platform/
│   │   ├── macos/             # Swift InputMethodKit shell
│   │   └── windows/           # C++ TSF shell
│   ├── data/                  # Dictionary, autocorrect, layout files
│   ├── Cargo.toml             # Cargo workspace root
│   └── .mise.toml             # Project-scoped tooling
└── docs/                      # Documentation
```

## Prerequisites

- macOS: Xcode 15+ (for InputMethodKit)
- Windows: Visual Studio Build Tools 2022+ (for MSVC/TSF)
- [mise](https://mise.jdx.dev) — install with `brew install mise` (macOS) or `curl https://mise.run | sh`

## Setup

```bash
cd app
mise install                                    # installs Rust, Node (project-scoped)
mise exec -- cargo binstall -y tauri-cli        # install Tauri CLI
cd crates/tauri-settings/src-frontend && npm install && cd ../../..  # frontend deps
```

## Run

```bash
mise run test          # run all Rust tests (56 tests: phonetic, unibijoy, national, buffer, layout, etc.)
mise run dev           # launch settings/onboarding UI in dev mode
mise run lint          # format + clippy
mise run build-engine  # build FFI static library (release)
```

## macOS IME

The app has two parts: the **IME** (background input method) and the **Settings UI** (Tauri app).

### Development

#### Build & install the IME locally

```bash
./platform/macos/build-and-install.sh
```

Compiles the Rust engine + Swift IMK shell, embeds the Settings app, and copies the `.app` to `~/Library/Input Methods/`. You then need to add the input source manually:

1. System Settings → Keyboard → Input Sources → Edit → "+" → Bangla → Bangla Keyboard
2. Press **Globe key** or **Ctrl+Space** to switch to Bangla

Re-run the script after any engine (Rust) or Swift changes. The first time the IME starts and onboarding hasn't been completed, it automatically launches the Settings/Onboarding window.

#### Run the Settings UI (dev mode)

```bash
mise run dev
```

Launches the Tauri settings/onboarding window with hot-reload. Independent of the IME — use this for iterating on the UI.

#### Clean slate

```bash
killall BanglaKeyboard 2>/dev/null
rm -rf ~/Library/Input\ Methods/BanglaKeyboard.app
rm -rf ~/Library/Application\ Support/dev.banglakeyboard.settings
```

The input source disappears after logging out and back in.

### Production installer

#### Build the .pkg

```bash
./platform/macos/package/build-pkg.sh
```

Builds the `.app` (if needed) and packages it into `target/macos-ime/BanglaKeyboard.pkg`. The installer:
- Copies the `.app` to `~/Library/Input Methods/`
- Runs `bangla-keyboard-register` to auto-register the input source
- Shows a conclusion page telling the user to grant permission and log out/in

No manual System Settings steps needed for the end user.

## Windows IME

### Development

```bash
mise run build-windows  # build Rust engine + CMake TSF DLL
regsvr32 BanglaKeyboard.dll  # register manually for dev testing
# Settings → Time & Language → Language & Region → add Bengali keyboard
# Press Win+Space to switch
```

Unregister with `regsvr32 /u BanglaKeyboard.dll`.

### Production installer

```bash
# Requires WiX Toolset v4+ (https://wixtoolset.org)
wix build -d BuildDir=target/release -d DataDir=data platform/windows/installer/Package.wxs -o target/BanglaKeyboard.msi
```

The `.msi` registers the TSF DLL automatically — no manual steps for the end user. After install, press **Win+Space** to switch to Bangla.

Uninstall via **Settings → Apps → Bangla Keyboard → Uninstall**, or `msiexec /x BanglaKeyboard.msi`.

## Deploy

1. Tag a release: `git tag v0.1.0 && git push --tags`
2. CI builds `.pkg` (macOS) and `.msi` (Windows) installers via GitHub Actions
3. Artifacts are uploaded to the GitHub Release automatically
