use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
pub struct Autocorrect {
    entries: HashMap<String, String>,
}

impl Autocorrect {
    pub fn load(data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let path = data_dir.join("autocorrect").join("autocorrect.json");
        if !path.exists() {
            return Ok(Self::default());
        }
        let json = std::fs::read_to_string(&path)?;
        // OpenBangla format: flat { "key": "value" } map
        let entries: HashMap<String, String> = serde_json::from_str(&json)?;
        Ok(Self { entries })
    }

    pub fn correct(&self, input: &str) -> Option<&str> {
        self.entries.get(input).map(|s| s.as_str())
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_autocorrect() {
        let ac = Autocorrect::default();
        assert_eq!(ac.entry_count(), 0);
        assert!(ac.correct("test").is_none());
    }
}
