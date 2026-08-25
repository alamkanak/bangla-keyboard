pub mod autocorrect;
pub mod buffer;
pub mod dictionary;
pub mod layout;
pub mod national;
pub mod phonetic;
pub mod unibijoy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    Phonetic,
    UniBijoy,
    National,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitAction {
    Commit,
    /// Replace the previously committed character with the new one
    CommitReplaceLast,
    /// Delete `backspace_count` chars from committed text, then commit new text
    CommitReplaceN { backspace_count: usize },
    UpdatePreview,
    Nothing,
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub text: String,
    pub is_from_dictionary: bool,
}

pub struct Engine {
    mode: LayoutMode,
    phonetic: phonetic::PhoneticEngine,
    unibijoy: unibijoy::UniBijoyEngine,
    national: national::NationalEngine,
    buffer: buffer::ComposingBuffer,
}

impl Engine {
    pub fn new(data_dir: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let phonetic = phonetic::PhoneticEngine::load(data_dir)?;
        let unibijoy = unibijoy::UniBijoyEngine::new();
        let national = national::NationalEngine::new();
        let buffer = buffer::ComposingBuffer::new();

        Ok(Self {
            mode: LayoutMode::Phonetic,
            phonetic,
            unibijoy,
            national,
            buffer,
        })
    }

    pub fn set_mode(&mut self, mode: LayoutMode) {
        self.mode = mode;
        self.buffer.clear();
    }

    pub fn mode(&self) -> LayoutMode {
        self.mode
    }

    pub fn handle_key(&mut self, key: char, shift: bool) -> CommitAction {
        self.handle_key_full(key, shift, false)
    }

    pub fn handle_key_full(&mut self, key: char, shift: bool, altgr: bool) -> CommitAction {
        match self.mode {
            LayoutMode::Phonetic => self.handle_phonetic_key(key),
            LayoutMode::UniBijoy => self.handle_unibijoy_key(key, shift),
            LayoutMode::National => self.handle_national_key(key, shift, altgr),
        }
    }

    pub fn handle_backspace(&mut self) -> CommitAction {
        match self.mode {
            LayoutMode::Phonetic => {
                if self.buffer.is_empty() {
                    return CommitAction::Nothing;
                }
                self.buffer.pop();
                if self.buffer.is_empty() {
                    return CommitAction::Commit;
                }
                let bangla = self.phonetic.transliterate(self.buffer.raw());
                self.buffer.set_preview(bangla);
                CommitAction::UpdatePreview
            }
            LayoutMode::UniBijoy => CommitAction::Nothing,
            LayoutMode::National => self.handle_national_backspace(),
        }
    }

    pub fn handle_enter(&mut self) -> Option<String> {
        self.commit_current()
    }

    pub fn handle_space(&mut self) -> Option<String> {
        self.commit_current()
    }

    pub fn preview(&self) -> &str {
        self.buffer.preview()
    }

    pub fn raw_input(&self) -> &str {
        self.buffer.raw()
    }

    pub fn candidates(&self) -> &[Candidate] {
        self.buffer.candidates()
    }

    pub fn select_candidate(&mut self, index: usize) -> Option<String> {
        if let Some(candidate) = self.buffer.candidates().get(index) {
            let text = candidate.text.clone();
            self.buffer.clear();
            Some(text)
        } else {
            None
        }
    }

    pub fn is_composing(&self) -> bool {
        !self.buffer.is_empty()
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.unibijoy.reset();
        self.national.reset();
    }

    fn commit_current(&mut self) -> Option<String> {
        if self.buffer.is_empty() {
            return None;
        }
        let text = self.buffer.preview().to_string();
        self.buffer.clear();
        Some(text)
    }

    fn handle_phonetic_key(&mut self, key: char) -> CommitAction {
        self.buffer.push(key);
        let bangla = self.phonetic.transliterate(self.buffer.raw());
        let candidates = self.phonetic.get_candidates(self.buffer.raw(), &bangla);
        self.buffer.set_preview(bangla);
        self.buffer.set_candidates(candidates);
        CommitAction::UpdatePreview
    }

    fn handle_unibijoy_key(&mut self, key: char, shift: bool) -> CommitAction {
        if let Some((ch, replace_last)) = self.unibijoy.process_key(key, shift) {
            if replace_last {
                self.buffer.clear();
                self.buffer.set_preview(ch);
                CommitAction::Commit
            } else if ch == "\u{09CD}" {
                self.buffer.set_preview(ch);
                CommitAction::UpdatePreview
            } else {
                let mut output = String::new();
                if !self.buffer.is_empty() {
                    output.push_str(self.buffer.preview());
                    self.buffer.clear();
                }
                output.push_str(&ch);
                self.buffer.set_preview(output);
                CommitAction::Commit
            }
        } else {
            CommitAction::Nothing
        }
    }

    fn handle_national_key(&mut self, key: char, shift: bool, altgr: bool) -> CommitAction {
        use national::NationalAction;
        match self.national.process_key(key, shift, altgr) {
            NationalAction::Commit(text) => {
                self.buffer.set_preview(text);
                CommitAction::Commit
            }
            NationalAction::ReplaceAndCommit { backspace_count, text } => {
                self.buffer.set_preview(text);
                CommitAction::CommitReplaceN { backspace_count }
            }
            NationalAction::Nothing => CommitAction::Nothing,
        }
    }

    fn handle_national_backspace(&mut self) -> CommitAction {
        use national::NationalAction;
        match self.national.handle_backspace() {
            NationalAction::ReplaceAndCommit { backspace_count, text } => {
                self.buffer.set_preview(text);
                CommitAction::CommitReplaceN { backspace_count }
            }
            NationalAction::Nothing => CommitAction::Nothing,
            NationalAction::Commit(text) => {
                self.buffer.set_preview(text);
                CommitAction::Commit
            }
        }
    }
}
