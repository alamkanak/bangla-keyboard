use crate::Candidate;

pub struct ComposingBuffer {
    raw: String,
    preview: String,
    candidates: Vec<Candidate>,
}

impl ComposingBuffer {
    pub fn new() -> Self {
        Self {
            raw: String::new(),
            preview: String::new(),
            candidates: Vec::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.raw.push(ch);
    }

    pub fn push_bangla(&mut self, text: &str) {
        self.preview.push_str(text);
        self.raw.push_str(text);
    }

    pub fn pop(&mut self) {
        self.raw.pop();
        // Preview will be recalculated by the engine
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn preview(&self) -> &str {
        &self.preview
    }

    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    pub fn set_preview(&mut self, preview: String) {
        self.preview = preview;
    }

    pub fn set_candidates(&mut self, candidates: Vec<Candidate>) {
        self.candidates = candidates;
    }

    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    pub fn clear(&mut self) {
        self.raw.clear();
        self.preview.clear();
        self.candidates.clear();
    }
}

impl Default for ComposingBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer_is_empty() {
        let buf = ComposingBuffer::new();
        assert!(buf.is_empty());
        assert_eq!(buf.raw(), "");
        assert_eq!(buf.preview(), "");
        assert!(buf.candidates().is_empty());
    }

    #[test]
    fn push_and_pop() {
        let mut buf = ComposingBuffer::new();
        buf.push('a');
        buf.push('m');
        assert_eq!(buf.raw(), "am");
        assert!(!buf.is_empty());

        buf.pop();
        assert_eq!(buf.raw(), "a");

        buf.pop();
        assert!(buf.is_empty());
    }

    #[test]
    fn push_bangla() {
        let mut buf = ComposingBuffer::new();
        buf.push_bangla("আ");
        assert_eq!(buf.preview(), "আ");
    }

    #[test]
    fn set_preview_and_candidates() {
        let mut buf = ComposingBuffer::new();
        buf.push('a');
        buf.set_preview("আ".to_string());
        assert_eq!(buf.preview(), "আ");

        buf.set_candidates(vec![
            Candidate {
                text: "আমি".to_string(),
                is_from_dictionary: true,
            },
            Candidate {
                text: "আমার".to_string(),
                is_from_dictionary: true,
            },
        ]);
        assert_eq!(buf.candidates().len(), 2);
    }

    #[test]
    fn clear_resets_everything() {
        let mut buf = ComposingBuffer::new();
        buf.push('t');
        buf.set_preview("ত".to_string());
        buf.set_candidates(vec![Candidate {
            text: "ত".to_string(),
            is_from_dictionary: false,
        }]);

        buf.clear();
        assert!(buf.is_empty());
        assert_eq!(buf.preview(), "");
        assert!(buf.candidates().is_empty());
    }
}
