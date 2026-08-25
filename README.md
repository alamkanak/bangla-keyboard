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

### Build & install the IME

```bash
./platform/macos/build-and-install.sh
```

This compiles the Rust engine + Swift IMK shell, bundles data files, and copies the `.app` to `~/Library/Input Methods/`. After install:

1. Open **System Settings → Keyboard → Input Sources → Edit**
2. Click **"+"**, select **"Bangla"** on the left, then **"Bangla Keyboard"** on the right
3. Press **Ctrl+Space** (or globe key) to switch between English and Bangla
4. Click the input method icon in the menu bar for **"Open Settings…"**

If you change engine code (Rust) or the Swift shell, re-run the script to rebuild.

### Run the Settings UI (dev mode)

```bash
mise run dev
```

This launches the Tauri settings/onboarding window in dev mode with hot-reload. The settings app is independent of the IME — the IME handles keyboard input, the settings app configures preferences.

## Windows IME

```bash
mise run build-windows  # build Rust engine, then use CMake for the TSF DLL
# After building:
# 1. regsvr32 BanglaKeyboard.dll
# 2. Settings → Time & Language → Language & Region → add Bengali keyboard
# 3. Press Ctrl+Space to toggle
```

## Deploy

1. Tag a release: `git tag v0.1.0 && git push --tags`
2. CI builds `.pkg` (macOS) and `.msi` (Windows) installers via GitHub Actions
3. Artifacts are uploaded to the GitHub Release automatically
