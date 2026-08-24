# Tech Stack

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Settings UI (Tauri)                  │
│              Rust backend + HTML/CSS/JS frontend        │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────┼─────────────────────────────────┐
│                   Rust Core Engine                       │
│  Phonetic transliteration · UniBijoy key map · Dictionary│
│              Exposed via C FFI (cbindgen)                │
└──────┬─────────────────┼─────────────────────┬───────────┘
       │                 │                     │
┌──────▼──────┐          │          ┌──────────▼──────────┐
│ macOS Shell │          │          │   Windows Shell     │
│ Swift/ObjC  │          │          │   C++ (COM)         │
│ InputMethod │          │          │   Text Services     │
│ Kit (IMK)   │          │          │   Framework (TSF)   │
└─────────────┘          │          └─────────────────────┘
                         │
              ┌──────────▼──────────┐
              │   Dictionary Data   │
              │   zstd-compressed   │
              │   ~150k Bangla words│
              └─────────────────────┘
```

## Stack by Layer

| Layer           | macOS                         | Windows                         | Shared                            |
| --------------- | ----------------------------- | ------------------------------- | --------------------------------- |
| **IME Shell**   | Swift + InputMethodKit        | C++ + TSF                       | —                                 |
| **Core Engine** | —                             | —                               | Rust (static lib via `cbindgen`)  |
| **Settings UI** | —                             | —                               | Tauri (Rust + HTML/CSS/JS)        |
| **Dictionary**  | —                             | —                               | Rust + zstd-compressed data files |
| **Build**       | Xcode + `cargo`               | CMake + `cargo`                 | `cargo` workspace                 |
| **CI/CD**       | GitHub Actions (macOS runner) | GitHub Actions (Windows runner) | GitHub Actions                    |
| **Testing**     | XCTest + `cargo test`         | GoogleTest + `cargo test`       | `cargo test`                      |
| **Packaging**   | `pkgbuild` → `.pkg`           | WiX Toolset → `.msi`            | GitHub Releases                   |

## Core Engine (Rust)

The engine is a platform-agnostic Rust library that handles all keyboard logic:

- **Phonetic transliteration** — pattern-matching rules converting Roman input to Bangla Unicode (Avro-compatible)
- **UniBijoy fixed layout** — direct key-to-Unicode character mapping
- **Dictionary & autocorrect** — word prediction, phonetic disambiguation, user-editable entries
- **Composing buffer** — manages the current input session state, candidate list, and commit logic

### Cargo Workspace

```
bangla-keyboard/
├── crates/
│   ├── engine-core/       # Transliteration, key maps, dictionary, buffer management
│   ├── engine-ffi/        # C FFI bindings generated via cbindgen
│   └── tauri-settings/    # Tauri app for settings/preferences UI
├── platform/
│   ├── macos/             # Xcode project: Swift/ObjC IMK shell
│   └── windows/           # CMake project: C++ TSF shell
├── data/
│   ├── dictionary/        # Bangla word list (zstd-compressed)
│   ├── autocorrect/       # Autocorrect entries (JSON)
│   └── layouts/           # Layout definition files (JSON)
├── docs/
└── Cargo.toml             # Workspace root
```

### Key Rust Dependencies

| Crate                   | Purpose                                   |
| ----------------------- | ----------------------------------------- |
| `cbindgen`              | Generate C headers from Rust for FFI      |
| `zstd`                  | Dictionary compression/decompression      |
| `serde` / `serde_json`  | Layout and autocorrect data serialization |
| `unicode-normalization` | NFC/NFD normalization for Bangla text     |
| `tauri`                 | Settings UI framework                     |

## Platform Shells

### macOS — InputMethodKit (IMK)

- **Language:** Swift with Objective-C bridging (IMK requires `NSObject` subclassing)
- **Key classes:** `IMKServer`, `IMKInputController`, `IMKCandidates`
- **Build:** Xcode project linking the Rust static library
- **Registration:** `.app` bundle installed in `~/Library/Input Methods/`, registered as a system input source via `Info.plist`
- **Hotkey:** Configurable toggle between Bangla and system keyboard

### Windows — Text Services Framework (TSF)

- **Language:** C++ (COM-based)
- **Key interfaces:** `ITfTextInputProcessorEx`, `ITfKeyEventSink`, `ITfComposition`
- **Build:** CMake project linking the Rust static library
- **Registration:** COM DLL registered via `regsvr32`, installs as a system text input processor
- **Hotkey:** Configurable toggle, defaults to `Ctrl+Space`

## Settings UI (Tauri)

A standalone Tauri app for managing preferences, shared across both platforms:

- Layout viewer (on-screen keyboard showing active layout)
- Toggle between UniBijoy and Phonetic modes
- Dictionary and autocorrect management (add/edit/remove entries)
- Hotkey configuration
- Auto-update check

The Tauri backend communicates directly with `engine-core` (same Rust process), and the frontend uses vanilla HTML/CSS/JS (no framework needed for a simple settings panel).

## Testing Strategy (TDD)

| Scope               | Tool                                 | What's Tested                                                                       |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| **Unit (Rust)**     | `cargo test`                         | Every transliteration rule, key mapping, dictionary lookup, buffer state transition |
| **FFI Integration** | Swift XCTest / C++ GoogleTest        | Rust FFI bindings produce correct output when called from platform code             |
| **Settings UI**     | Tauri WebDriver (via `tauri-driver`) | Layout viewer renders correctly, settings persist, dictionary CRUD works            |
| **E2E (manual)**    | Platform-specific                    | Type Bangla text in real apps (TextEdit, Notepad), verify Unicode output            |

### Test-first workflow

1. Write a failing test for the transliteration rule or key mapping
2. Implement the minimum Rust code to pass
3. Verify FFI works by running platform-specific integration tests
4. Refactor, keeping all tests green

## Development Environment

### Prerequisites

- **mise:** [mise.jdx.dev](https://mise.jdx.dev) — manages Rust, Node, and cargo tools per-project
- **macOS:** Xcode 15+ (for InputMethodKit SDK and Swift compiler)
- **Windows:** Visual Studio Build Tools 2022+ (for MSVC and Windows SDK)

All other tools (Rust stable, Node, cbindgen, Tauri CLI) are installed project-locally via `mise install` using the `.mise.toml` at the repo root.

### Local Dev Commands

```bash
# First-time setup
mise install
mise exec -- cargo binstall -y cbindgen tauri-cli

# Run all Rust tests
mise run test

# Build macOS IME
mise run build-macos

# Build Windows IME
mise run build-windows

# Run Tauri settings app in dev mode
mise run dev

# Lint & format
mise run lint

# Or use mise exec for ad-hoc commands
mise exec -- cargo test -- --nocapture
```

### Pre-commit Hooks

- `mise exec -- cargo fmt --check`
- `mise exec -- cargo clippy -- -D warnings`
- `mise exec -- cargo test --workspace`

## Packaging & Distribution

| Platform | Format           | Tool                        | Channel         |
| -------- | ---------------- | --------------------------- | --------------- |
| macOS    | `.pkg` installer | `pkgbuild` + `productbuild` | GitHub Releases |
| Windows  | `.msi` installer | WiX Toolset                 | GitHub Releases |

### CI/CD (GitHub Actions)

- **On PR:** `cargo test`, `cargo clippy`, `cargo fmt --check`
- **On tag:** Build platform installers, create GitHub Release with artifacts
- **Matrix:** macOS (latest) + Windows (latest) runners

## Reference Projects

| Project                                                                  | Relevance                                                         |
| ------------------------------------------------------------------------ | ----------------------------------------------------------------- |
| [OpenBangla Keyboard](https://github.com/OpenBangla/OpenBangla-Keyboard) | Bangla IME for Linux; uses Rust core ("riti") + C++/Qt UI + iBus  |
| [RIME (librime)](https://github.com/rime/librime)                        | Cross-platform IME engine in C++; plugin architecture for schemas |
| [Avro Keyboard](https://www.omicronlab.com/avro-keyboard.html)           | Original Bangla phonetic IME; reference for transliteration rules |
