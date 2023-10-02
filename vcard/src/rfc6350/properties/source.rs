use super::super::values::Source;
use crate::common::{VCardProperty, VCardValue};

pub struct SourceProperty {
    sources: Vec<Source>,
}

impl SourceProperty {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    pub fn add(&mut self, source: Source) {
        self.sources.push(source);
    }
}

impl VCardProperty for SourceProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for source in &self.sources {
            output.push_str(&source.format_value());
        }
        output
    }
}
