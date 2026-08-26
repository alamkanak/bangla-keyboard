#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppPreferences {
    onboarding_complete: bool,
    language: String,
    layout: String,
    theme: String,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            onboarding_complete: false,
            language: "en".into(),
            layout: "phonetic".into(),
            theme: "dark".into(),
        }
    }
}

struct AppState {
    prefs: Mutex<AppPreferences>,
    prefs_path: PathBuf,
}

impl AppState {
    fn load(prefs_path: PathBuf) -> Self {
        let prefs = fs::read_to_string(&prefs_path)
            .ok()
            .and_then(|s| serde_json::from_str::<AppPreferences>(&s).ok())
            .unwrap_or_default();
        Self {
            prefs: Mutex::new(prefs),
            prefs_path,
        }
    }

    fn save(&self) {
        let prefs = self.prefs.lock().unwrap();
        if let Some(parent) = self.prefs_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&self.prefs_path, serde_json::to_string_pretty(&*prefs).unwrap());
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LayoutInfo {
    mode: String,
    name: String,
}

#[tauri::command]
fn get_layouts() -> Vec<LayoutInfo> {
    vec![
        LayoutInfo {
            mode: "phonetic".to_string(),
            name: "Phonetic (Avro)".to_string(),
        },
        LayoutInfo {
            mode: "unibijoy".to_string(),
            name: "UniBijoy".to_string(),
        },
        LayoutInfo {
            mode: "national".to_string(),
            name: "National (Jatiya)".to_string(),
        },
    ]
}

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn get_preferences(state: tauri::State<AppState>) -> AppPreferences {
    state.prefs.lock().unwrap().clone()
}

#[tauri::command]
fn complete_onboarding(
    state: tauri::State<AppState>,
    language: String,
    layout: String,
    theme: String,
) {
    let mut prefs = state.prefs.lock().unwrap();
    prefs.onboarding_complete = true;
    prefs.language = language;
    prefs.layout = layout;
    prefs.theme = theme;
    drop(prefs);
    state.save();
}

#[tauri::command]
fn update_preference(state: tauri::State<AppState>, key: String, value: String) {
    let mut prefs = state.prefs.lock().unwrap();
    match key.as_str() {
        "language" => prefs.language = value,
        "layout" => prefs.layout = value,
        "theme" => prefs.theme = value,
        _ => return,
    }
    drop(prefs);
    state.save();
}

/// Check if the IME is registered and enabled in the OS.
#[tauri::command]
fn check_ime_status() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").map_err(|e| e.to_string())?;
        let exe = std::path::PathBuf::from(&home)
            .join("Library/Input Methods/BanglaKeyboard.app/Contents/MacOS/bangla-keyboard-register");

        if !exe.exists() {
            return Ok("not-installed".into());
        }

        let output = std::process::Command::new(&exe)
            .arg("--check-status")
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok("enabled".into())
        } else {
            Ok("installed-not-enabled".into())
        }
    }
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
        match hkcr.open_subkey("CLSID\\{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}") {
            Ok(_) => Ok("enabled".into()),
            Err(_) => Ok("not-installed".into()),
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Ok("unsupported-platform".into())
    }
}

/// Attempt to enable the IME on macOS by running the registration commands.
#[tauri::command]
fn enable_ime() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").map_err(|e| e.to_string())?;
        let exe = PathBuf::from(&home)
            .join("Library/Input Methods/BanglaKeyboard.app/Contents/MacOS/bangla-keyboard-register");

        if !exe.exists() {
            return Err("BanglaKeyboard.app not found in ~/Library/Input Methods/".into());
        }

        // Register input source
        let reg = std::process::Command::new(&exe)
            .arg("--register-input-source")
            .output()
            .map_err(|e| e.to_string())?;

        if !reg.status.success() {
            return Err("Failed to register input source".into());
        }

        // Enable input source
        let en = std::process::Command::new(&exe)
            .arg("--enable-input-source")
            .output()
            .map_err(|e| e.to_string())?;

        if !en.status.success() {
            return Err("Failed to enable input source".into());
        }

        Ok("enabled".into())
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok("handled-by-installer".into())
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let config_dir = app
                .path()
                .app_config_dir()
                .unwrap_or_else(|_| PathBuf::from("."));
            let prefs_path = config_dir.join("preferences.json");
            app.manage(AppState::load(prefs_path));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_layouts,
            get_version,
            get_preferences,
            complete_onboarding,
            update_preference,
            check_ime_status,
            enable_ime,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ime_status_returns_valid_status() {
        let result = check_ime_status();
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(
            ["enabled", "not-installed", "installed-not-enabled", "unsupported-platform"]
                .contains(&status.as_str()),
            "Unexpected status: {status}"
        );
    }

    #[test]
    fn app_preferences_default_values() {
        let prefs = AppPreferences::default();
        assert!(!prefs.onboarding_complete);
        assert_eq!(prefs.language, "en");
        assert_eq!(prefs.layout, "phonetic");
        assert_eq!(prefs.theme, "dark");
    }

    #[test]
    fn app_state_loads_defaults_for_missing_file() {
        let state = AppState::load(PathBuf::from("/tmp/nonexistent-bangla-test-prefs.json"));
        let prefs = state.prefs.lock().unwrap();
        assert!(!prefs.onboarding_complete);
    }
}
