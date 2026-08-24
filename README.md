# Bangla Keyboard

A macOS + Windows input method supporting UniBijoy and Phonetic keyboard layouts for typing in Bangla.

## Prerequisites

- macOS: Xcode 15+ (for InputMethodKit)
- Windows: Visual Studio Build Tools 2022+ (for MSVC/TSF)
- [mise](https://mise.jdx.dev) — install with `brew install mise` (macOS) or `curl https://mise.run | sh`

## Setup

1. `git clone <repo-url> && cd bangla-keyboard`
2. `mise install` — installs Rust, Node, cbindgen, Tauri CLI (project-scoped)
3. `mise exec -- cargo binstall -y cbindgen tauri-cli` — install cargo tools

## Run

1. `mise run test` — run all Rust tests
2. `mise run dev` — launch settings UI in dev mode
3. `mise run lint` — format and lint

## Debug

1. `mise run build-macos` — build the macOS IME via Xcode
2. The IME appears in System Settings → Keyboard → Input Sources → add "Bangla Keyboard"
3. `mise exec -- cargo test -- --nocapture` — engine debug output

## Deploy

1. Tag a release: `git tag v0.1.0 && git push --tags`
2. CI builds `.pkg` (macOS) and `.msi` (Windows) installers via GitHub Actions
3. Artifacts are uploaded to the GitHub Release automatically
