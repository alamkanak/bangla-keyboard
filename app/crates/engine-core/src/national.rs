use std::collections::HashMap;

const HASANTA: &str = "\u{09CD}";
const ZWNJ: &str = "\u{200C}";

// Bangla character range constants
const CONSONANT_START: char = '\u{0995}';
const CONSONANT_END: char = '\u{09B9}';
const EXTRA_CONSONANT_START: char = '\u{09DC}';
const EXTRA_CONSONANT_END: char = '\u{09DF}';

const B_CHANDRA: &str = "\u{0981}";
const B_RA: &str = "র";

// Vowel signs (kars)
const AA_KAR: &str = "\u{09BE}";
const I_KAR: &str = "\u{09BF}";
const II_KAR: &str = "\u{09C0}";
const U_KAR: &str = "\u{09C1}";
const UU_KAR: &str = "\u{09C2}";
const RRI_KAR: &str = "\u{09C3}";
const E_KAR: &str = "\u{09C7}";
const OI_KAR: &str = "\u{09C8}";
const O_KAR: &str = "\u{09CB}";
const OU_KAR: &str = "\u{09CC}";
const RR_KAR: &str = "\u{09C4}";
const AU_LEN: &str = "\u{09D7}";

// Full vowels
const B_AA: &str = "আ";
const B_I: &str = "ই";
const B_II: &str = "ঈ";
const B_U: &str = "উ";
const B_UU: &str = "ঊ";
const B_RRI: &str = "ঋ";
const B_E: &str = "এ";
const B_OI: &str = "ঐ";
const B_O: &str = "ও";
const B_OU: &str = "ঔ";

const TRACK_LIMIT: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NationalAction {
    /// Commit text directly, no buffering
    Commit(String),
    /// Replace `backspace_count` chars then emit text
    ReplaceAndCommit { backspace_count: usize, text: String },
    /// Nothing to output (key not mapped)
    Nothing,
}

pub struct NationalEngine {
    normal_map: HashMap<char, &'static str>,
    shift_map: HashMap<char, &'static str>,
    altgr_map: HashMap<char, &'static str>,
    shift_altgr_map: HashMap<char, &'static str>,
    vowel_sign_to_full: HashMap<&'static str, &'static str>,
    /// Buffer of recently output characters for context-aware processing
    output_buffer: Vec<String>,
    /// Whether we're in dead-key state (after hasanta or at word start)
    dead_key: bool,
}

impl NationalEngine {
    pub fn new() -> Self {
        Self {
            normal_map: Self::build_normal_map(),
            shift_map: Self::build_shift_map(),
            altgr_map: Self::build_altgr_map(),
            shift_altgr_map: Self::build_shift_altgr_map(),
            vowel_sign_to_full: Self::build_vowel_map(),
            output_buffer: Vec::new(),
            dead_key: true, // start in dead-key state (word start)
        }
    }

    pub fn reset(&mut self) {
        self.output_buffer.clear();
        self.dead_key = true;
    }

    pub fn process_key(
        &mut self,
        key: char,
        shift: bool,
        altgr: bool,
    ) -> NationalAction {
        let mapped = self.map_key(key, shift, altgr);
        let mapped = match mapped {
            Some(s) => s,
            None => return NationalAction::Nothing,
        };

        // Word-breaking keys reset state
        if is_word_break(&mapped) {
            self.push_output(&mapped);
            self.dead_key = true;
            return NationalAction::Commit(mapped);
        }

        // Hasanta + Hasanta → ZWNJ (double hasanta produces explicit halant display)
        if mapped == HASANTA && self.last_output() == Some(HASANTA) {
            self.pop_output();
            let text = ZWNJ.to_string();
            self.push_output(&text);
            self.dead_key = true;
            return NationalAction::ReplaceAndCommit {
                backspace_count: 1,
                text,
            };
        }

        // Dead-key vowel logic: at word start or after hasanta, vowel kar → full vowel
        if self.is_vowel_sign(&mapped) {
            if self.dead_key || self.output_buffer.is_empty() {
                // Word start: vowel kar → full vowel
                if let Some(full) = self.vowel_sign_to_full.get(mapped.as_str()) {
                    let text = full.to_string();
                    self.push_output(&text);
                    self.dead_key = false;
                    return NationalAction::Commit(text);
                }
            }

            if self.last_output() == Some(HASANTA) {
                // After hasanta: remove hasanta, emit full vowel
                if let Some(&full) = self.vowel_sign_to_full.get(mapped.as_str()) {
                    self.pop_output();
                    let text = full.to_string();
                    self.push_output(&text);
                    self.dead_key = false;
                    return NationalAction::ReplaceAndCommit {
                        backspace_count: 1,
                        text,
                    };
                }
            }

            // Chandrabindu interaction: kar after chandrabindu → reorder
            if self.last_output() == Some(B_CHANDRA) {
                self.pop_output();
                let text = format!("{}{}", mapped, B_CHANDRA);
                let full_text = text.clone();
                self.push_output(&mapped);
                self.push_output(B_CHANDRA);
                self.dead_key = false;
                return NationalAction::ReplaceAndCommit {
                    backspace_count: 1,
                    text: full_text,
                };
            }

            // Normal vowel sign after consonant
            self.push_output(&mapped);
            self.dead_key = false;
            return NationalAction::Commit(mapped);
        }

        // Hasanta key
        if mapped == HASANTA {
            self.push_output(&mapped);
            self.dead_key = false;
            return NationalAction::Commit(mapped);
        }

        // Reph logic: র + hasanta before a consonant → reph reordering
        if Self::is_consonant_str(&mapped) && self.should_form_reph() {
            let reph_result = self.do_reph(&mapped);
            return reph_result;
        }

        // Regular consonant or other character
        if Self::is_consonant_str(&mapped) {
            self.push_output(&mapped);
            self.dead_key = false;
            return NationalAction::Commit(mapped);
        }

        // Any other character (punctuation, digits, etc.)
        self.push_output(&mapped);
        self.dead_key = is_non_bangla(&mapped);
        NationalAction::Commit(mapped)
    }

    pub fn handle_backspace(&mut self) -> NationalAction {
        if self.output_buffer.is_empty() {
            return NationalAction::Nothing;
        }

        let last = self.output_buffer.last().unwrap().clone();

        // If last was part of a conjunct (hasanta), remove the whole conjunct step
        if last == HASANTA {
            // Remove hasanta
            self.pop_output();
            self.dead_key = self.output_buffer.is_empty()
                || self.last_output().map_or(true, |s| is_non_bangla(s));
            return NationalAction::ReplaceAndCommit {
                backspace_count: 1,
                text: String::new(),
            };
        }

        // If the char before last is hasanta, we need to remove consonant+hasanta
        // (undo a conjunct formation step)
        if self.output_buffer.len() >= 2 {
            let second_last = self.output_buffer[self.output_buffer.len() - 2].clone();
            if second_last == HASANTA && Self::is_consonant_str(&last) {
                // Remove consonant + hasanta
                self.pop_output(); // consonant
                self.pop_output(); // hasanta
                self.dead_key = false;
                return NationalAction::ReplaceAndCommit {
                    backspace_count: 2,
                    text: String::new(),
                };
            }
        }

        // Regular single character removal
        self.pop_output();
        self.dead_key = self.output_buffer.is_empty()
            || self.last_output().map_or(true, |s| is_non_bangla(s));
        NationalAction::ReplaceAndCommit {
            backspace_count: 1,
            text: String::new(),
        }
    }

    /// Get the number of characters in the output buffer
    pub fn buffer_len(&self) -> usize {
        self.output_buffer.len()
    }

    /// Check if we're at word start (dead key state)
    pub fn is_at_word_start(&self) -> bool {
        self.dead_key
    }

    fn map_key(&self, key: char, shift: bool, altgr: bool) -> Option<String> {
        let lookup_key = if shift && !altgr {
            key
        } else if !shift && !altgr {
            key
        } else {
            key.to_ascii_lowercase()
        };

        let result = if shift && altgr {
            self.shift_altgr_map
                .get(&lookup_key)
                .or_else(|| self.altgr_map.get(&lookup_key))
        } else if altgr {
            self.altgr_map.get(&lookup_key)
        } else if shift {
            self.shift_map.get(&key)
        } else {
            self.normal_map.get(&key)
        };

        result.map(|s| s.to_string())
    }

    fn last_output(&self) -> Option<&str> {
        self.output_buffer.last().map(|s| s.as_str())
    }

    fn last_n_output(&self, n: usize) -> Option<&str> {
        if self.output_buffer.len() >= n {
            Some(&self.output_buffer[self.output_buffer.len() - n])
        } else {
            None
        }
    }

    fn push_output(&mut self, s: &str) {
        self.output_buffer.push(s.to_string());
        if self.output_buffer.len() > TRACK_LIMIT {
            self.output_buffer.remove(0);
        }
    }

    fn pop_output(&mut self) -> Option<String> {
        self.output_buffer.pop()
    }

    fn is_vowel_sign(&self, s: &str) -> bool {
        matches!(
            s,
            AA_KAR | I_KAR | II_KAR | U_KAR | UU_KAR | RRI_KAR | E_KAR | OI_KAR | O_KAR
                | OU_KAR | RR_KAR | AU_LEN
        )
    }

    fn is_consonant_str(s: &str) -> bool {
        if let Some(ch) = s.chars().next() {
            if s.chars().count() == 1 {
                return matches!(ch, CONSONANT_START..=CONSONANT_END | EXTRA_CONSONANT_START..=EXTRA_CONSONANT_END);
            }
        }
        false
    }

    fn should_form_reph(&self) -> bool {
        // Reph forms when: previous output is hasanta AND the one before that is র
        if self.output_buffer.len() < 2 {
            return false;
        }
        self.last_output() == Some(HASANTA)
            && self.last_n_output(2) == Some(B_RA)
    }

    fn do_reph(&mut self, consonant: &str) -> NationalAction {
        // Remove র + hasanta from buffer, then figure out where to place reph
        // Reph goes: র্ + consonant → consonant + র্ (reordered above)
        self.pop_output(); // hasanta
        self.pop_output(); // র

        // Calculate how many chars to backspace (র + hasanta = 2 rendered chars)
        let reph = format!("{}{}", B_RA, HASANTA);
        let text = format!("{}{}", reph, consonant);
        self.push_output(B_RA);
        self.push_output(HASANTA);
        self.push_output(consonant);
        self.dead_key = false;

        // We need to backspace the র্ and re-emit র্ + consonant
        // The font/shaper handles the visual reordering of reph
        NationalAction::ReplaceAndCommit {
            backspace_count: 2,
            text,
        }
    }

    fn build_vowel_map() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        m.insert(AA_KAR, B_AA);
        m.insert(I_KAR, B_I);
        m.insert(II_KAR, B_II);
        m.insert(U_KAR, B_U);
        m.insert(UU_KAR, B_UU);
        m.insert(RRI_KAR, B_RRI);
        m.insert(E_KAR, B_E);
        m.insert(OI_KAR, B_OI);
        m.insert(O_KAR, B_O);
        m.insert(OU_KAR, B_OU);
        m
    }

    fn build_normal_map() -> HashMap<char, &'static str> {
        let mut m = HashMap::new();
        m.insert('`', "\u{200C}"); // ZWNJ
        m.insert('1', "১");
        m.insert('2', "২");
        m.insert('3', "৩");
        m.insert('4', "৪");
        m.insert('5', "৫");
        m.insert('6', "৬");
        m.insert('7', "৭");
        m.insert('8', "৮");
        m.insert('9', "৯");
        m.insert('0', "০");
        m.insert('-', "-");
        m.insert('=', "=");
        m.insert('q', "ঙ");
        m.insert('w', "য");
        m.insert('e', "ড");
        m.insert('r', "প");
        m.insert('t', "ট");
        m.insert('y', "চ");
        m.insert('u', "জ");
        m.insert('i', "হ");
        m.insert('o', "গ");
        m.insert('p', "ড়");
        m.insert('[', "[");
        m.insert(']', "]");
        m.insert('\\', "\\");
        m.insert('a', RRI_KAR);  // ৃ
        m.insert('s', U_KAR);    // ু
        m.insert('d', I_KAR);    // ি
        m.insert('f', "ব");
        m.insert('g', HASANTA);  // ্
        m.insert('h', AA_KAR);   // া
        m.insert('j', "ক");
        m.insert('k', "ত");
        m.insert('l', "দ");
        m.insert(';', ";");
        m.insert('\'', "'");
        m.insert('z', B_CHANDRA); // ঁ
        m.insert('x', O_KAR);    // ো
        m.insert('c', E_KAR);    // ে
        m.insert('v', "র");
        m.insert('b', "ন");
        m.insert('n', "স");
        m.insert('m', "ম");
        m.insert(',', ",");
        m.insert('.', ".");
        m.insert('/', "/");
        m
    }

    fn build_shift_map() -> HashMap<char, &'static str> {
        let mut m = HashMap::new();
        m.insert('~', "\u{200D}"); // ZWJ
        m.insert('!', "!");
        m.insert('@', "@");
        m.insert('#', "#");
        m.insert('$', "$");
        m.insert('%', "%");
        m.insert('^', "^");
        m.insert('&', "&");
        m.insert('*', "*");
        m.insert('(', "(");
        m.insert(')', ")");
        m.insert('_', "_");
        m.insert('+', "+");
        m.insert('Q', "ং");
        m.insert('W', "য়");
        m.insert('E', "ঢ");
        m.insert('R', "ফ");
        m.insert('T', "ঠ");
        m.insert('Y', "ছ");
        m.insert('U', "ঝ");
        m.insert('I', "ঞ");
        m.insert('O', "ঘ");
        m.insert('P', "ঢ়");
        m.insert('{', "{");
        m.insert('}', "}");
        m.insert('|', "|");
        m.insert('A', AU_LEN);   // ৗ
        m.insert('S', UU_KAR);   // ূ
        m.insert('D', II_KAR);   // ী
        m.insert('F', "ভ");
        m.insert('G', "।");
        m.insert('H', "অ");
        m.insert('J', "খ");
        m.insert('K', "থ");
        m.insert('L', "ধ");
        m.insert(':', ":");
        m.insert('"', "\"");
        m.insert('Z', "\u{0983}"); // ঃ (visarga)
        m.insert('X', OU_KAR);    // ৌ
        m.insert('C', OI_KAR);    // ৈ
        m.insert('V', "ল");
        m.insert('B', "ণ");
        m.insert('N', "ষ");
        m.insert('M', "শ");
        m.insert('<', "<");
        m.insert('>', ">");
        m.insert('?', "?");
        m
    }

    fn build_altgr_map() -> HashMap<char, &'static str> {
        let mut m = HashMap::new();
        m.insert('`', "\u{200C}"); // ZWNJ
        m.insert('1', "\u{09F4}"); // ৴
        m.insert('2', "\u{09F5}"); // ৵
        m.insert('3', "\u{09F6}"); // ৶
        m.insert('4', "\u{09F3}"); // ৳ Taka
        m.insert('5', "\u{09F7}"); // ৷
        m.insert('6', "\u{09F8}"); // ৸
        m.insert('q', "\u{09E2}"); // ৢ
        m.insert('e', "\u{09C4}"); // ৄ
        m.insert('i', "\u{09BD}"); // ঽ
        m.insert('a', "ঋ");
        m.insert('s', "উ");
        m.insert('d', "ই");
        m.insert('f', "\u{09F0}"); // ৰ
        m.insert('g', "\u{0965}"); // ॥ double dari
        m.insert('h', "আ");
        m.insert('l', "\u{098C}"); // ঌ
        m.insert('z', "\u{09FA}"); // ৺
        m.insert('x', "ও");
        m.insert('c', "এ");
        m
    }

    fn build_shift_altgr_map() -> HashMap<char, &'static str> {
        let mut m = HashMap::new();
        m.insert('`', "\u{200D}"); // ZWJ
        m.insert('4', "\u{09F2}"); // ৲
        m.insert('6', "ৎ");
        m.insert('q', "\u{09E3}"); // ৣ
        m.insert('a', "\u{09E0}"); // ৠ
        m.insert('s', "ঊ");
        m.insert('d', "ঈ");
        m.insert('f', "\u{09F1}"); // ৱ
        m.insert('l', "\u{09E1}"); // ৡ
        m.insert('x', "ঔ");
        m.insert('c', "ঐ");
        m
    }
}

impl Default for NationalEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn is_word_break(s: &str) -> bool {
    matches!(s, " " | "\n" | "\r" | "\t" | "।" | "\u{0965}")
}

fn is_non_bangla(s: &str) -> bool {
    if let Some(ch) = s.chars().next() {
        if s.chars().count() == 1 {
            // Bangla range: U+0980 - U+09FF, plus danda U+0964-0965
            return !matches!(ch, '\u{0964}'..='\u{0965}' | '\u{0980}'..='\u{09FF}' | '\u{200C}'..='\u{200D}');
        }
    }
    // Multi-char strings like আ are Bangla
    s.chars().all(|c| !matches!(c, '\u{0964}'..='\u{0965}' | '\u{0980}'..='\u{09FF}' | '\u{200C}'..='\u{200D}'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_consonant_mappings() {
        let mut eng = NationalEngine::new();
        // Reset dead_key state by emitting a consonant first (simulating mid-word)
        eng.dead_key = false;
        eng.output_buffer.push("ক".to_string());

        assert_eq!(
            eng.process_key('j', false, false),
            NationalAction::Commit("ক".to_string())
        );
        assert_eq!(
            eng.process_key('k', false, false),
            NationalAction::Commit("ত".to_string())
        );
    }

    #[test]
    fn shifted_consonant_mappings() {
        let mut eng = NationalEngine::new();
        eng.dead_key = false;
        eng.output_buffer.push("ক".to_string());

        assert_eq!(
            eng.process_key('J', true, false),
            NationalAction::Commit("খ".to_string())
        );
        assert_eq!(
            eng.process_key('K', true, false),
            NationalAction::Commit("থ".to_string())
        );
        assert_eq!(
            eng.process_key('E', true, false),
            NationalAction::Commit("ঢ".to_string())
        );
    }

    #[test]
    fn vowel_sign_after_consonant() {
        let mut eng = NationalEngine::new();
        // Type ক then া → কা
        eng.process_key('j', false, false); // emits ক, sets dead_key=false
        let result = eng.process_key('h', false, false); // া (aa-kar)
        assert_eq!(result, NationalAction::Commit(AA_KAR.to_string()));
    }

    #[test]
    fn vowel_at_word_start_produces_full_vowel() {
        let mut eng = NationalEngine::new();
        // At word start, া-kar key → আ
        let result = eng.process_key('h', false, false);
        assert_eq!(result, NationalAction::Commit(B_AA.to_string()));
    }

    #[test]
    fn vowel_after_hasanta_produces_full_vowel() {
        let mut eng = NationalEngine::new();
        // Type ক then hasanta then vowel sign → full vowel replaces hasanta
        eng.process_key('j', false, false); // ক
        eng.process_key('g', false, false); // ্ (hasanta)
        let result = eng.process_key('h', false, false); // া after hasanta → আ
        assert_eq!(
            result,
            NationalAction::ReplaceAndCommit {
                backspace_count: 1,
                text: B_AA.to_string()
            }
        );
    }

    #[test]
    fn hasanta_between_consonants_forms_conjunct() {
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        let h = eng.process_key('g', false, false); // ্
        assert_eq!(h, NationalAction::Commit(HASANTA.to_string()));
        let c = eng.process_key('k', false, false); // ত → ক্ত
        // Consonant after hasanta (not র) → normal commit
        assert_eq!(c, NationalAction::Commit("ত".to_string()));
    }

    #[test]
    fn double_hasanta_produces_zwnj() {
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        eng.process_key('g', false, false); // ্
        let result = eng.process_key('g', false, false); // ্ again → ZWNJ
        assert_eq!(
            result,
            NationalAction::ReplaceAndCommit {
                backspace_count: 1,
                text: ZWNJ.to_string()
            }
        );
    }

    #[test]
    fn reph_reordering() {
        let mut eng = NationalEngine::new();
        eng.process_key('v', false, false); // র
        eng.process_key('g', false, false); // ্ (hasanta)
        let result = eng.process_key('k', false, false); // ত → র্ + ত (reph)
        assert_eq!(
            result,
            NationalAction::ReplaceAndCommit {
                backspace_count: 2,
                text: format!("{}{}{}", B_RA, HASANTA, "ত")
            }
        );
    }

    #[test]
    fn backspace_removes_single_char() {
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        let result = eng.handle_backspace();
        assert_eq!(
            result,
            NationalAction::ReplaceAndCommit {
                backspace_count: 1,
                text: String::new()
            }
        );
        assert!(eng.output_buffer.is_empty());
    }

    #[test]
    fn backspace_removes_hasanta() {
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        eng.process_key('g', false, false); // ্
        let result = eng.handle_backspace();
        assert_eq!(
            result,
            NationalAction::ReplaceAndCommit {
                backspace_count: 1,
                text: String::new()
            }
        );
        // Buffer should have only ক
        assert_eq!(eng.output_buffer.len(), 1);
        assert_eq!(eng.output_buffer[0], "ক");
    }

    #[test]
    fn backspace_empty_buffer() {
        let mut eng = NationalEngine::new();
        let result = eng.handle_backspace();
        assert_eq!(result, NationalAction::Nothing);
    }

    #[test]
    fn backspace_removes_conjunct_step() {
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        eng.process_key('g', false, false); // ্
        eng.process_key('k', false, false); // ত (forming ক্ত)
        // Backspace should remove ত + ্ (the conjunct step)
        let result = eng.handle_backspace();
        assert_eq!(
            result,
            NationalAction::ReplaceAndCommit {
                backspace_count: 2,
                text: String::new()
            }
        );
        // Should have only ক left
        assert_eq!(eng.output_buffer.len(), 1);
        assert_eq!(eng.output_buffer[0], "ক");
    }

    #[test]
    fn bangla_digits() {
        let mut eng = NationalEngine::new();
        let result = eng.process_key('1', false, false);
        assert_eq!(result, NationalAction::Commit("১".to_string()));
    }

    #[test]
    fn altgr_layer() {
        let mut eng = NationalEngine::new();
        // AltGr + 4 → ৳ (Taka sign)
        let result = eng.process_key('4', false, true);
        assert_eq!(result, NationalAction::Commit("\u{09F3}".to_string()));
    }

    #[test]
    fn altgr_full_vowels() {
        let mut eng = NationalEngine::new();
        // AltGr + a → ঋ (full vowel, not kar)
        let result = eng.process_key('a', false, true);
        assert_eq!(result, NationalAction::Commit("ঋ".to_string()));
    }

    #[test]
    fn shift_altgr_layer() {
        let mut eng = NationalEngine::new();
        // Shift+AltGr + 6 → ৎ
        let result = eng.process_key('6', true, true);
        assert_eq!(result, NationalAction::Commit("ৎ".to_string()));
    }

    #[test]
    fn chandrabindu_key() {
        let mut eng = NationalEngine::new();
        let result = eng.process_key('z', false, false);
        assert_eq!(result, NationalAction::Commit(B_CHANDRA.to_string()));
    }

    #[test]
    fn visarga_key() {
        let mut eng = NationalEngine::new();
        let result = eng.process_key('Z', true, false);
        assert_eq!(result, NationalAction::Commit("\u{0983}".to_string()));
    }

    #[test]
    fn dari_key() {
        let mut eng = NationalEngine::new();
        let result = eng.process_key('G', true, false);
        assert_eq!(result, NationalAction::Commit("।".to_string()));
    }

    #[test]
    fn e_kar_at_word_start_gives_full_e() {
        let mut eng = NationalEngine::new();
        let result = eng.process_key('c', false, false);
        assert_eq!(result, NationalAction::Commit(B_E.to_string()));
    }

    #[test]
    fn o_kar_at_word_start_gives_full_o() {
        let mut eng = NationalEngine::new();
        let result = eng.process_key('x', false, false);
        assert_eq!(result, NationalAction::Commit(B_O.to_string()));
    }

    #[test]
    fn oi_kar_at_word_start_gives_full_oi() {
        let mut eng = NationalEngine::new();
        let result = eng.process_key('C', true, false);
        assert_eq!(result, NationalAction::Commit(B_OI.to_string()));
    }

    #[test]
    fn ou_kar_at_word_start_gives_full_ou() {
        let mut eng = NationalEngine::new();
        let result = eng.process_key('X', true, false);
        assert_eq!(result, NationalAction::Commit(B_OU.to_string()));
    }

    #[test]
    fn word_break_resets_dead_key() {
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        assert!(!eng.dead_key);
        eng.process_key('G', true, false); // । (dari = word break)
        assert!(eng.dead_key);
    }

    #[test]
    fn typing_sequence_ami() {
        // আমি: অ + ম + ি  → but in National layout:
        // আ = H(shift) gives অ. Hmm, let me trace:
        // At word start: 'h' gives AA_KAR which → full vowel আ (dead key logic)
        // Then 'm' gives ম (consonant)
        // Then 'd' gives ি (vowel sign after consonant)
        let mut eng = NationalEngine::new();
        let r1 = eng.process_key('h', false, false); // → আ (word start)
        assert_eq!(r1, NationalAction::Commit("আ".to_string()));

        let r2 = eng.process_key('m', false, false); // → ম
        assert_eq!(r2, NationalAction::Commit("ম".to_string()));

        let r3 = eng.process_key('d', false, false); // → ি after consonant
        assert_eq!(r3, NationalAction::Commit(I_KAR.to_string()));
    }

    #[test]
    fn typing_sequence_bangla() {
        // বাংলা: ব + া + ং + ল + া
        // f=ব, h=া, Q(shift)=ং, V(shift)=ল, h=া
        let mut eng = NationalEngine::new();
        eng.dead_key = false; // assume mid-sentence
        eng.output_buffer.push("x".to_string()); // dummy to avoid word-start

        let r1 = eng.process_key('f', false, false); // ব
        assert_eq!(r1, NationalAction::Commit("ব".to_string()));

        let r2 = eng.process_key('h', false, false); // া after consonant
        assert_eq!(r2, NationalAction::Commit(AA_KAR.to_string()));

        let r3 = eng.process_key('Q', true, false); // ং
        assert_eq!(r3, NationalAction::Commit("ং".to_string()));

        let r4 = eng.process_key('V', true, false); // ল
        assert_eq!(r4, NationalAction::Commit("ল".to_string()));

        let r5 = eng.process_key('h', false, false); // া after consonant
        assert_eq!(r5, NationalAction::Commit(AA_KAR.to_string()));
    }

    #[test]
    fn chandrabindu_reordering_with_kar() {
        // If chandrabindu is typed then a vowel kar is typed, reorder: kar + chandrabindu
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        eng.process_key('z', false, false); // ঁ
        let result = eng.process_key('h', false, false); // া → should reorder to া + ঁ
        assert_eq!(
            result,
            NationalAction::ReplaceAndCommit {
                backspace_count: 1,
                text: format!("{}{}", AA_KAR, B_CHANDRA)
            }
        );
    }

    #[test]
    fn i_kar_after_consonant() {
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        let result = eng.process_key('d', false, false); // ি
        assert_eq!(result, NationalAction::Commit(I_KAR.to_string()));
    }

    #[test]
    fn u_kar_after_consonant() {
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        let result = eng.process_key('s', false, false); // ু
        assert_eq!(result, NationalAction::Commit(U_KAR.to_string()));
    }

    #[test]
    fn multiple_conjuncts() {
        // ক্ষ = ক + ্ + ষ
        let mut eng = NationalEngine::new();
        eng.process_key('j', false, false); // ক
        eng.process_key('g', false, false); // ্
        let result = eng.process_key('N', true, false); // ষ
        assert_eq!(result, NationalAction::Commit("ষ".to_string()));
    }
}
