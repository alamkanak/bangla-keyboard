use crate::Candidate;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AvroData {
    layout: AvroLayout,
}

#[derive(Debug, Deserialize)]
struct AvroLayout {
    vowel: String,
    consonant: String,
    casesensitive: String,
    number: String,
    patterns: Vec<Pattern>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Pattern {
    find: String,
    replace: String,
    #[serde(default)]
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    #[serde(default)]
    matches: Vec<Match>,
    replace: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Match {
    #[serde(rename = "type")]
    match_type: String,
    scope: String,
    #[serde(default)]
    value: Option<String>,
}

pub struct PhoneticEngine {
    patterns: Vec<Pattern>,
    vowels: String,
    consonants: String,
    casesensitive: String,
    numbers: String,
    autocorrect: HashMap<String, String>,
    dictionary: HashMap<String, Vec<String>>,
    #[allow(dead_code)]
    suffix: HashMap<String, String>,
}

impl PhoneticEngine {
    pub fn load(data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let avro_path = data_dir.join("avrophonetic.json");
        let avro_json = std::fs::read_to_string(&avro_path)?;
        let avro_data: AvroData = serde_json::from_str(&avro_json)?;

        let autocorrect = Self::load_autocorrect(data_dir)?;
        let dictionary = Self::load_dictionary(data_dir)?;
        let suffix = Self::load_suffix(data_dir)?;

        Ok(Self {
            patterns: avro_data.layout.patterns,
            vowels: avro_data.layout.vowel,
            consonants: avro_data.layout.consonant,
            casesensitive: avro_data.layout.casesensitive,
            numbers: avro_data.layout.number,
            autocorrect,
            dictionary,
            suffix,
        })
    }

    fn load_autocorrect(
        data_dir: &Path,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let path = data_dir.join("autocorrect").join("autocorrect.json");
        if !path.exists() {
            return Ok(HashMap::new());
        }
        let json = std::fs::read_to_string(&path)?;
        // OpenBangla format: flat { "key": "value" } map
        let map: HashMap<String, String> = serde_json::from_str(&json)?;
        Ok(map)
    }

    fn load_dictionary(
        data_dir: &Path,
    ) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let path = data_dir.join("dictionary").join("dictionary.json");
        if !path.exists() {
            return Ok(HashMap::new());
        }
        let json = std::fs::read_to_string(&path)?;
        let data: HashMap<String, Vec<String>> = serde_json::from_str(&json)?;
        Ok(data)
    }

    fn load_suffix(data_dir: &Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let path = data_dir.join("dictionary").join("suffix.json");
        if !path.exists() {
            return Ok(HashMap::new());
        }
        let json = std::fs::read_to_string(&path)?;
        let map: HashMap<String, String> = serde_json::from_str(&json)?;
        Ok(map)
    }

    pub fn transliterate(&self, input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }

        // Check autocorrect first
        if let Some(corrected) = self.autocorrect.get(input) {
            return corrected.clone();
        }

        self.do_transliterate(input)
    }

    fn do_transliterate(&self, input: &str) -> String {
        let mut output = String::new();
        let _input_lower = if self.is_case_sensitive_input(input) {
            input.to_string()
        } else {
            input.to_lowercase()
        };
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let mut matched = false;

            // Try patterns from longest to shortest
            for pattern in &self.patterns {
                let find_len = pattern.find.len();
                if i + find_len > chars.len() {
                    continue;
                }

                let slice = &input[i..i + find_len];
                let find_to_match = if self.should_case_match(&pattern.find) {
                    slice.to_string()
                } else {
                    slice.to_lowercase()
                };

                let pattern_to_match = if self.should_case_match(&pattern.find) {
                    pattern.find.clone()
                } else {
                    pattern.find.to_lowercase()
                };

                if find_to_match == pattern_to_match {
                    let replacement =
                        self.apply_rules(&pattern.rules, &pattern.replace, input, i, find_len);
                    output.push_str(&replacement);
                    i += find_len;
                    matched = true;
                    break;
                }
            }

            if !matched {
                output.push(chars[i]);
                i += 1;
            }
        }

        output
    }

    fn should_case_match(&self, pattern: &str) -> bool {
        for ch in pattern.chars() {
            if ch.is_ascii_alphabetic()
                && self
                    .casesensitive
                    .contains(ch.to_lowercase().next().unwrap_or(ch))
            {
                return true;
            }
        }
        false
    }

    fn is_case_sensitive_input(&self, input: &str) -> bool {
        for ch in input.chars() {
            if self.casesensitive.contains(ch) {
                return true;
            }
        }
        true // Always preserve case for matching
    }

    fn apply_rules(
        &self,
        rules: &[Rule],
        default_replace: &str,
        input: &str,
        pos: usize,
        find_len: usize,
    ) -> String {
        for rule in rules {
            if self.check_rule_matches(&rule.matches, input, pos, find_len) {
                return rule.replace.clone();
            }
        }
        default_replace.to_string()
    }

    fn check_rule_matches(
        &self,
        matches: &[Match],
        input: &str,
        pos: usize,
        find_len: usize,
    ) -> bool {
        for m in matches {
            if !self.check_single_match(m, input, pos, find_len) {
                return false;
            }
        }
        true
    }

    fn check_single_match(&self, m: &Match, input: &str, pos: usize, find_len: usize) -> bool {
        let is_negated = m.scope.starts_with('!');
        let scope = if is_negated { &m.scope[1..] } else { &m.scope };

        let result = match m.match_type.as_str() {
            "prefix" => self.check_scope(scope, input, pos, true, m.value.as_deref()),
            "suffix" => self.check_scope(scope, input, pos + find_len, false, m.value.as_deref()),
            _ => false,
        };

        if is_negated {
            !result
        } else {
            result
        }
    }

    fn check_scope(
        &self,
        scope: &str,
        input: &str,
        pos: usize,
        is_prefix: bool,
        value: Option<&str>,
    ) -> bool {
        match scope {
            "vowel" => {
                let ch = if is_prefix {
                    self.get_prev_char(input, pos)
                } else {
                    self.get_next_char(input, pos)
                };
                ch.is_some_and(|c| self.vowels.contains(c.to_lowercase().next().unwrap_or(c)))
            }
            "consonant" => {
                let ch = if is_prefix {
                    self.get_prev_char(input, pos)
                } else {
                    self.get_next_char(input, pos)
                };
                ch.is_some_and(|c| {
                    self.consonants
                        .contains(c.to_lowercase().next().unwrap_or(c))
                })
            }
            "number" => {
                let ch = if is_prefix {
                    self.get_prev_char(input, pos)
                } else {
                    self.get_next_char(input, pos)
                };
                ch.is_some_and(|c| self.numbers.contains(c))
            }
            "punctuation" => {
                let ch = if is_prefix {
                    self.get_prev_char(input, pos)
                } else {
                    self.get_next_char(input, pos)
                };
                if is_prefix && pos == 0 {
                    return true; // Beginning of string counts as punctuation
                }
                ch.is_some_and(|c| {
                    !self.vowels.contains(c.to_lowercase().next().unwrap_or(c))
                        && !self
                            .consonants
                            .contains(c.to_lowercase().next().unwrap_or(c))
                        && !self.numbers.contains(c)
                })
            }
            "exact" => {
                if let Some(val) = value {
                    if is_prefix {
                        if pos == 0 {
                            return false;
                        }
                        let start = if pos >= val.len() { pos - val.len() } else { 0 };
                        let prev = &input[start..pos];
                        prev == val
                    } else {
                        if pos >= input.len() {
                            return false;
                        }
                        let end = std::cmp::min(pos + val.len(), input.len());
                        let next = &input[pos..end];
                        next == val
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn get_prev_char(&self, input: &str, pos: usize) -> Option<char> {
        if pos == 0 {
            return None;
        }
        input[..pos].chars().last()
    }

    fn get_next_char(&self, input: &str, pos: usize) -> Option<char> {
        input[pos..].chars().next()
    }

    pub fn get_candidates(&self, input: &str, transliterated: &str) -> Vec<Candidate> {
        let mut candidates = Vec::new();

        // Always include the direct transliteration
        candidates.push(Candidate {
            text: transliterated.to_string(),
            is_from_dictionary: false,
        });

        // Look up dictionary words
        let key = self.get_dictionary_key(input);
        if let Some(words) = self.dictionary.get(&key) {
            for word in words.iter().take(9) {
                if word != transliterated {
                    candidates.push(Candidate {
                        text: word.clone(),
                        is_from_dictionary: true,
                    });
                }
            }
        }

        candidates
    }

    fn get_dictionary_key(&self, input: &str) -> String {
        input
            .chars()
            .next()
            .map(|c| c.to_lowercase().to_string())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_AVRO_JSON: &str = include_str!("../tests/test_avrophonetic.json");

    fn create_test_data_dir() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();

        let avro_path = dir.path().join("avrophonetic.json");
        std::fs::write(&avro_path, TEST_AVRO_JSON).unwrap();

        let ac_dir = dir.path().join("autocorrect");
        std::fs::create_dir_all(&ac_dir).unwrap();
        std::fs::write(ac_dir.join("autocorrect.json"), r#"{}"#).unwrap();

        let dict_dir = dir.path().join("dictionary");
        std::fs::create_dir_all(&dict_dir).unwrap();
        std::fs::write(dict_dir.join("dictionary.json"), r#"{}"#).unwrap();

        dir
    }

    fn engine() -> (tempfile::TempDir, PhoneticEngine) {
        let dir = create_test_data_dir();
        let engine = PhoneticEngine::load(dir.path()).unwrap();
        (dir, engine)
    }

    #[test]
    fn basic_consonants() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate("k"), "ক");
        assert_eq!(eng.transliterate("g"), "গ");
        assert_eq!(eng.transliterate("t"), "ত");
        assert_eq!(eng.transliterate("d"), "দ");
        assert_eq!(eng.transliterate("p"), "প");
        assert_eq!(eng.transliterate("b"), "ব");
        assert_eq!(eng.transliterate("m"), "ম");
        assert_eq!(eng.transliterate("n"), "ন");
        assert_eq!(eng.transliterate("l"), "ল");
        assert_eq!(eng.transliterate("s"), "স");
        assert_eq!(eng.transliterate("h"), "হ");
        assert_eq!(eng.transliterate("j"), "জ");
    }

    #[test]
    fn aspirated_consonants() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate("kh"), "খ");
        assert_eq!(eng.transliterate("gh"), "ঘ");
        assert_eq!(eng.transliterate("ch"), "ছ");
        assert_eq!(eng.transliterate("jh"), "ঝ");
        assert_eq!(eng.transliterate("th"), "থ");
        assert_eq!(eng.transliterate("dh"), "ধ");
        assert_eq!(eng.transliterate("ph"), "ফ");
        assert_eq!(eng.transliterate("bh"), "ভ");
        assert_eq!(eng.transliterate("sh"), "শ");
    }

    #[test]
    fn retroflex_consonants() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate("T"), "ট");
        assert_eq!(eng.transliterate("D"), "ড");
        assert_eq!(eng.transliterate("N"), "ণ");
        assert_eq!(eng.transliterate("Th"), "ঠ");
        assert_eq!(eng.transliterate("Dh"), "ঢ");
        assert_eq!(eng.transliterate("Sh"), "ষ");
    }

    #[test]
    fn standalone_vowels() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate("a"), "আ");
        assert_eq!(eng.transliterate("i"), "ই");
        assert_eq!(eng.transliterate("u"), "উ");
        assert_eq!(eng.transliterate("e"), "এ");
        assert_eq!(eng.transliterate("o"), "অ");
        assert_eq!(eng.transliterate("ee"), "ঈ");
        assert_eq!(eng.transliterate("oo"), "উ");
        assert_eq!(eng.transliterate("OI"), "ঐ");
        assert_eq!(eng.transliterate("OU"), "ঔ");
    }

    #[test]
    fn vowel_signs_after_consonant() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate("ka"), "কা");
        assert_eq!(eng.transliterate("ki"), "কি");
        assert_eq!(eng.transliterate("ku"), "কু");
        assert_eq!(eng.transliterate("ke"), "কে");
    }

    #[test]
    fn word_ami() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate("ami"), "আমি");
    }

    #[test]
    fn bangla_digits() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate("0"), "০");
        assert_eq!(eng.transliterate("1"), "১");
        assert_eq!(eng.transliterate("9"), "৯");
    }

    #[test]
    fn ng_produces_anusvara() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate("ng"), "ং");
    }

    #[test]
    fn empty_input() {
        let (_dir, eng) = engine();
        assert_eq!(eng.transliterate(""), "");
    }
}
