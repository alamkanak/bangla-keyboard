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

> **Choose your platform:** [Mac](#mac) · [Windows](#windows). The Windows section below is fully self-contained (fresh-install → running IME) — Windows users can skip the shared *Setup* / *Run* blocks below.

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

## Mac

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

## Windows

Complete setup for a **fresh Windows 10/11 install**. Run every command in an **elevated PowerShell** (Right-click Start → *Terminal (Admin)*), in order, from the repo root unless noted.

### Prerequisites (one-time)

1. Install Git — `winget install --id Git.Git -e --source winget`
2. Install VS 2022 Build Tools with the C++ workload — `winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--passive --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"`
3. Install CMake (added to PATH) — `winget install --id Kitware.CMake -e`
4. Install the WebView2 Runtime (required by Tauri) — `winget install --id Microsoft.EdgeWebView2Runtime -e`
5. Install .NET SDK 8 (required by WiX v4) — `winget install --id Microsoft.DotNet.SDK.8 -e`
6. Install mise — `winget install --id jdx.mise -e`
7. Close and reopen the elevated PowerShell so the new tools land on `PATH`.
8. Install WiX Toolset v5 (needed for the `.msi` installer; v7 requires accepting the paid OSMF EULA — pin to v5) — `dotnet tool install --global wix --version 5.0.2`

### Development build & install

9. Enter the app workspace — `cd app`
10. Trust the project's mise config — `mise trust`
11. Install Rust + Node + cargo-binstall pinned in `.mise.toml` — `mise install`
12. Install the Tauri CLI — `mise exec -- cargo binstall -y tauri-cli`
13. Install the Settings UI frontend deps — `pushd crates\tauri-settings\src-frontend; npm install; popd`
14. Run all Rust tests to verify the toolchain — `mise run test`
15. Build the Rust engine (release) — `mise run build-engine`
16. Configure the TSF DLL build — pick `ARM64` on Windows on ARM (Copilot+ PCs, Surface Pro X), `x64` on Intel/AMD: `cmake -S platform\windows -B platform\windows\build -A ARM64`
17. Compile the TSF DLL (run inside a `mise` shell so cargo is on PATH for CMake's Rust rebuild rule) — `mise exec -- cmake --build platform\windows\build --config Release`
18. Register the DLL for local testing — `regsvr32 platform\windows\build\Release\BanglaKeyboard.dll`
19. Launch the Settings/Onboarding UI in dev mode (optional, hot-reload) — `mise run dev`
20. Add the input: **Settings → Time & Language → Language & Region → Bengali → Add a keyboard → Bangla Keyboard**, then press **Win+Space** to switch.

Unregister after dev testing — `regsvr32 /u platform\windows\build\Release\BanglaKeyboard.dll`

### Production installer (`.msi`)

21. Build the frontend + Tauri Settings exe (release) — `pushd crates\tauri-settings; mise exec -- cargo tauri build --no-bundle; popd`
22. Open a **new** elevated PowerShell so `%USERPROFILE%\.dotnet\tools` (where `wix` lives) is on `PATH`, then `cd` back into `app`.
23. One-time: register the WiX Util extension (used for recursive AppData cleanup on uninstall) — `wix extension add -g WixToolset.Util.wixext/5.0.2`
24. Build the MSI — `wix build -ext WixToolset.Util.wixext -d DllDir=platform\windows\build\Release -d SettingsDir=target\release -d DataDir=data platform\windows\installer\Package.wxs -o target\BanglaKeyboard.msi`

The `.msi` registers the TSF DLL automatically — no manual steps for the end user. After install, press **Win+Space** to switch to Bangla. Uninstall via **Settings → Apps → Bangla Keyboard → Uninstall**, or `msiexec /x target\BanglaKeyboard.msi`.

## Deploy

1. Tag a release: `git tag v0.1.0 && git push --tags`
2. CI builds `.pkg` (macOS) and `.msi` (Windows) installers via GitHub Actions
3. Artifacts are uploaded to the GitHub Release automatically
