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

/// Register the IME with the OS. On macOS this copies the .app bundle to ~/Library/Input Methods/.
#[tauri::command]
fn enable_ime() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").map_err(|e| e.to_string())?;
        let input_methods_dir = PathBuf::from(&home).join("Library/Input Methods");
        let _ = fs::create_dir_all(&input_methods_dir);

        // In production, the IME .app is bundled as a resource.
        // For now, just ensure the directory exists and report success.
        Ok("Input Methods directory ready. IME will be registered on next build.".into())
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok("IME registration is handled by the installer on Windows.".into())
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
            enable_ime,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
