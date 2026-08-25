use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
pub struct Dictionary {
    words: HashMap<String, Vec<String>>,
}

impl Dictionary {
    pub fn load(data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let path = data_dir.join("dictionary").join("dictionary.json");
        if !path.exists() {
            return Ok(Self {
                words: HashMap::new(),
            });
        }
        let json = std::fs::read_to_string(&path)?;
        let words: HashMap<String, Vec<String>> = serde_json::from_str(&json)?;
        Ok(Self { words })
    }

    pub fn search(&self, prefix: &str) -> Vec<&str> {
        let key = prefix
            .chars()
            .next()
            .map(|c| c.to_lowercase().to_string())
            .unwrap_or_default();

        self.words
            .get(&key)
            .map(|words| {
                words
                    .iter()
                    .filter(|w| w.starts_with(prefix))
                    .map(|w| w.as_str())
                    .take(20)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn contains(&self, word: &str) -> bool {
        let key = word
            .chars()
            .next()
            .map(|c| c.to_lowercase().to_string())
            .unwrap_or_default();

        self.words
            .get(&key)
            .map(|words| words.iter().any(|w| w == word))
            .unwrap_or(false)
    }

    pub fn word_count(&self) -> usize {
        self.words.values().map(|v| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_dictionary() {
        let dict = Dictionary::default();
        assert_eq!(dict.word_count(), 0);
        assert!(!dict.contains("test"));
        assert!(dict.search("t").is_empty());
    }
}
