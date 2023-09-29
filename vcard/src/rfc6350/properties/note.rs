use super::super::values::Note;
use crate::common::{VCardProperty, VCardValue};

pub struct NoteProperty {
    notes: Vec<Note>,
}

impl NoteProperty {
    pub fn new() -> Self {
        Self { notes: vec![] }
    }

    pub fn add(&mut self, note: Note) {
        self.notes.push(note);
    }
}

impl VCardProperty for NoteProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for note in &self.notes {
            output.push_str(&note.format_value());
        }
        output
    }
}
