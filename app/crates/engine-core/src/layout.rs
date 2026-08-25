use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDefinition {
    pub name: String,
    pub version: String,
    #[serde(rename = "type")]
    pub layout_type: String,
    pub key_mapping: LayoutKeyMapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutKeyMapping {
    pub normal: HashMap<String, String>,
    pub shift: HashMap<String, String>,
}

pub fn load_layout(path: &Path) -> Result<LayoutDefinition, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let layout: LayoutDefinition = serde_json::from_str(&json)?;
    Ok(layout)
}
