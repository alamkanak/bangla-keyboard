use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDefinition {
    pub name: String,
    pub version: String,
    #[serde(rename = "type")]
    pub layout_type: String,
    #[serde(default)]
    pub standard: Option<String>,
    #[serde(default)]
    pub developer: Option<String>,
    pub key_mapping: LayoutKeyMapping,
    #[serde(default)]
    pub numpad: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutKeyMapping {
    pub normal: HashMap<String, String>,
    pub shift: HashMap<String, String>,
    #[serde(default)]
    pub altgr: Option<HashMap<String, String>>,
    #[serde(default)]
    pub shift_altgr: Option<HashMap<String, String>>,
}

pub fn load_layout(path: &Path) -> Result<LayoutDefinition, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let layout: LayoutDefinition = serde_json::from_str(&json)?;
    Ok(layout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn load_layout_with_altgr() {
        let json = r#"{
            "name": "Test",
            "version": "1.0",
            "type": "fixed",
            "key_mapping": {
                "normal": {"a": "আ"},
                "shift": {"A": "অ"},
                "altgr": {"a": "ঋ"},
                "shift_altgr": {"a": "ৠ"}
            }
        }"#;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.json");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(json.as_bytes()).unwrap();

        let layout = load_layout(&path).unwrap();
        assert_eq!(layout.name, "Test");
        assert!(layout.key_mapping.altgr.is_some());
        assert!(layout.key_mapping.shift_altgr.is_some());
        assert_eq!(
            layout.key_mapping.altgr.unwrap().get("a").unwrap(),
            "ঋ"
        );
    }

    #[test]
    fn load_layout_without_altgr_backward_compat() {
        let json = r#"{
            "name": "UniBijoy",
            "version": "1.0",
            "type": "fixed",
            "key_mapping": {
                "normal": {"j": "ক"},
                "shift": {"J": "খ"}
            }
        }"#;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.json");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(json.as_bytes()).unwrap();

        let layout = load_layout(&path).unwrap();
        assert!(layout.key_mapping.altgr.is_none());
        assert!(layout.key_mapping.shift_altgr.is_none());
    }
}
