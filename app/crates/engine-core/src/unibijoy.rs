use std::collections::HashMap;

const HASANTA: &str = "\u{09CD}";

pub struct UniBijoyEngine {
    normal_map: HashMap<char, &'static str>,
    shift_map: HashMap<char, &'static str>,
    vowel_sign_to_full: HashMap<&'static str, &'static str>,
    last_output: Option<String>,
}

impl UniBijoyEngine {
    pub fn new() -> Self {
        let mut normal_map = HashMap::new();
        let mut shift_map = HashMap::new();

        // Normal (unshifted) mappings
        normal_map.insert('q', "ঙ");
        normal_map.insert('w', "য");
        normal_map.insert('e', "ড");
        normal_map.insert('r', "প");
        normal_map.insert('t', "ট");
        normal_map.insert('y', "চ");
        normal_map.insert('u', "জ");
        normal_map.insert('i', "হ");
        normal_map.insert('o', "গ");
        normal_map.insert('p', "ড়");
        normal_map.insert('a', "\u{09C3}"); // ৃ (rri-kar)
        normal_map.insert('s', "\u{09C1}"); // ু (u-kar)
        normal_map.insert('d', "\u{09BF}"); // ি (i-kar)
        normal_map.insert('f', "\u{09BE}"); // া (aa-kar)
        normal_map.insert('g', "\u{09CD}"); // ্ (hasanta)
        normal_map.insert('h', "ব");
        normal_map.insert('j', "ক");
        normal_map.insert('k', "ত");
        normal_map.insert('l', "দ");
        normal_map.insert(';', ";");
        normal_map.insert('z', "\u{00AA}"); // ª
        normal_map.insert('x', "ও");
        normal_map.insert('c', "\u{09C7}"); // ে (e-kar)
        normal_map.insert('v', "র");
        normal_map.insert('b', "ন");
        normal_map.insert('n', "স");
        normal_map.insert('m', "ম");

        // Shifted mappings
        shift_map.insert('Q', "ং");
        shift_map.insert('W', "য়");
        shift_map.insert('E', "ঢ");
        shift_map.insert('R', "ফ");
        shift_map.insert('T', "ঠ");
        shift_map.insert('Y', "ছ");
        shift_map.insert('U', "ঝ");
        shift_map.insert('I', "ঞ");
        shift_map.insert('O', "ঘ");
        shift_map.insert('P', "ঢ়");
        shift_map.insert('A', "\u{00A9}"); // ©
        shift_map.insert('S', "\u{09C2}"); // ূ (uu-kar)
        shift_map.insert('D', "\u{09C0}"); // ী (ii-kar)
        shift_map.insert('F', "অ");
        shift_map.insert('G', "।");
        shift_map.insert('H', "ভ");
        shift_map.insert('J', "খ");
        shift_map.insert('K', "থ");
        shift_map.insert('L', "ধ");
        shift_map.insert('Z', "\u{00A8}"); // ¨
        shift_map.insert('X', "\u{09CC}"); // ৌ (ou-kar)
        shift_map.insert('C', "\u{09C8}"); // ৈ (oi-kar)
        shift_map.insert('V', "ল");
        shift_map.insert('B', "ণ");
        shift_map.insert('N', "ষ");
        shift_map.insert('M', "শ");

        // Number row (normal = Bangla digits)
        normal_map.insert('0', "০");
        normal_map.insert('1', "১");
        normal_map.insert('2', "২");
        normal_map.insert('3', "৩");
        normal_map.insert('4', "৪");
        normal_map.insert('5', "৫");
        normal_map.insert('6', "৬");
        normal_map.insert('7', "৭");
        normal_map.insert('8', "৮");
        normal_map.insert('9', "৯");

        // Shifted number row
        shift_map.insert('!', "!");
        shift_map.insert('@', "@");
        shift_map.insert('#', "#");
        shift_map.insert('$', "৳"); // Taka sign
        shift_map.insert('%', "%");
        shift_map.insert('^', "ঁ"); // chandrabindu
        shift_map.insert('&', "&");
        shift_map.insert('*', "*");
        shift_map.insert('(', "(");
        shift_map.insert(')', ")");

        // Special keys
        normal_map.insert('\\', "ৎ"); // khanda-ta
        shift_map.insert('|', "ঃ"); // visarga
        normal_map.insert(',', ",");
        shift_map.insert('<', "<");
        shift_map.insert('>', ">");
        normal_map.insert('/', "/");
        shift_map.insert('?', "?");
        normal_map.insert('[', "[");
        normal_map.insert(']', "]");
        shift_map.insert('{', "{");
        shift_map.insert('}', "}");
        normal_map.insert('-', "-");
        shift_map.insert('_', "_");
        normal_map.insert('=', "=");
        shift_map.insert('+', "+");
        normal_map.insert('`', "`");
        shift_map.insert('~', "~");
        normal_map.insert('.', ".");
        normal_map.insert('\'', "'");
        shift_map.insert('"', "\"");

        Self {
            normal_map,
            shift_map,
            vowel_sign_to_full: Self::build_vowel_map(),
            last_output: None,
        }
    }

    fn build_vowel_map() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        m.insert("\u{09BE}", "আ"); // া → আ
        m.insert("\u{09BF}", "ই"); // ি → ই
        m.insert("\u{09C0}", "ঈ"); // ী → ঈ
        m.insert("\u{09C1}", "উ"); // ু → উ
        m.insert("\u{09C2}", "ঊ"); // ূ → ঊ
        m.insert("\u{09C3}", "ঋ"); // ৃ → ঋ
        m.insert("\u{09C7}", "এ"); // ে → এ
        m.insert("\u{09C8}", "ঐ"); // ৈ → ঐ
        m.insert("\u{09CB}", "ও"); // ো → ও
        m.insert("\u{09CC}", "ঔ"); // ৌ → ঔ
        m
    }

    fn is_vowel_sign(&self, s: &str) -> bool {
        self.vowel_sign_to_full.contains_key(s)
    }

    fn is_consonant(s: &str) -> bool {
        if let Some(ch) = s.chars().next() {
            matches!(ch, '\u{0995}'..='\u{09B9}' | '\u{09DC}'..='\u{09DF}')
        } else {
            false
        }
    }

    pub fn map_key(&self, key: char, shift: bool) -> Option<String> {
        let raw = if shift {
            self.shift_map
                .get(&key)
                .or_else(|| self.normal_map.get(&key))
        } else {
            self.normal_map.get(&key)
        }?;

        Some(raw.to_string())
    }

    /// Process a key with automatic vowel forming.
    /// Returns (output_string, should_replace_last) where should_replace_last
    /// means the previous committed character should be replaced.
    pub fn process_key(&mut self, key: char, shift: bool) -> Option<(String, bool)> {
        let raw = self.map_key(key, shift)?;

        // Rule: hasanta + vowel sign → full vowel (replace hasanta)
        if self.is_vowel_sign(&raw) {
            if let Some(ref last) = self.last_output {
                if last == HASANTA {
                    // Replace hasanta with full vowel
                    if let Some(&full) = self.vowel_sign_to_full.get(raw.as_str()) {
                        self.last_output = Some(full.to_string());
                        return Some((full.to_string(), true));
                    }
                }
                if !Self::is_consonant(last) && last != HASANTA {
                    // Vowel sign after non-consonant → full vowel
                    if let Some(&full) = self.vowel_sign_to_full.get(raw.as_str()) {
                        self.last_output = Some(full.to_string());
                        return Some((full.to_string(), false));
                    }
                }
            } else {
                // First character is a vowel sign → full vowel
                if let Some(&full) = self.vowel_sign_to_full.get(raw.as_str()) {
                    self.last_output = Some(full.to_string());
                    return Some((full.to_string(), false));
                }
            }
        }

        self.last_output = Some(raw.clone());
        Some((raw, false))
    }

    pub fn reset(&mut self) {
        self.last_output = None;
    }
}

impl Default for UniBijoyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_consonant_mappings() {
        let eng = UniBijoyEngine::new();
        assert_eq!(eng.map_key('j', false).unwrap(), "ক");
        assert_eq!(eng.map_key('k', false).unwrap(), "ত");
        assert_eq!(eng.map_key('l', false).unwrap(), "দ");
        assert_eq!(eng.map_key('h', false).unwrap(), "ব");
        assert_eq!(eng.map_key('v', false).unwrap(), "র");
        assert_eq!(eng.map_key('n', false).unwrap(), "স");
        assert_eq!(eng.map_key('m', false).unwrap(), "ম");
        assert_eq!(eng.map_key('b', false).unwrap(), "ন");
    }

    #[test]
    fn shifted_consonant_mappings() {
        let eng = UniBijoyEngine::new();
        assert_eq!(eng.map_key('J', true).unwrap(), "খ");
        assert_eq!(eng.map_key('K', true).unwrap(), "থ");
        assert_eq!(eng.map_key('L', true).unwrap(), "ধ");
        assert_eq!(eng.map_key('H', true).unwrap(), "ভ");
        assert_eq!(eng.map_key('R', true).unwrap(), "ফ");
        assert_eq!(eng.map_key('N', true).unwrap(), "ষ");
        assert_eq!(eng.map_key('M', true).unwrap(), "শ");
        assert_eq!(eng.map_key('B', true).unwrap(), "ণ");
    }

    #[test]
    fn vowel_signs() {
        let eng = UniBijoyEngine::new();
        assert_eq!(eng.map_key('f', false).unwrap(), "\u{09BE}"); // া
        assert_eq!(eng.map_key('d', false).unwrap(), "\u{09BF}"); // ি
        assert_eq!(eng.map_key('s', false).unwrap(), "\u{09C1}"); // ু
        assert_eq!(eng.map_key('c', false).unwrap(), "\u{09C7}"); // ে
        assert_eq!(eng.map_key('a', false).unwrap(), "\u{09C3}"); // ৃ
    }

    #[test]
    fn shifted_vowel_signs() {
        let eng = UniBijoyEngine::new();
        assert_eq!(eng.map_key('D', true).unwrap(), "\u{09C0}"); // ী
        assert_eq!(eng.map_key('S', true).unwrap(), "\u{09C2}"); // ূ
        assert_eq!(eng.map_key('C', true).unwrap(), "\u{09C8}"); // ৈ
        assert_eq!(eng.map_key('X', true).unwrap(), "\u{09CC}"); // ৌ
    }

    #[test]
    fn hasanta() {
        let eng = UniBijoyEngine::new();
        assert_eq!(eng.map_key('g', false).unwrap(), "\u{09CD}"); // ্
    }

    #[test]
    fn bangla_digits() {
        let eng = UniBijoyEngine::new();
        assert_eq!(eng.map_key('0', false).unwrap(), "০");
        assert_eq!(eng.map_key('1', false).unwrap(), "১");
        assert_eq!(eng.map_key('5', false).unwrap(), "৫");
        assert_eq!(eng.map_key('9', false).unwrap(), "৯");
    }

    #[test]
    fn special_characters() {
        let eng = UniBijoyEngine::new();
        assert_eq!(eng.map_key('\\', false).unwrap(), "ৎ"); // khanda-ta
        assert_eq!(eng.map_key('G', true).unwrap(), "।"); // dari
    }

    #[test]
    fn taka_sign() {
        let eng = UniBijoyEngine::new();
        assert_eq!(eng.map_key('$', true).unwrap(), "৳");
    }
}
