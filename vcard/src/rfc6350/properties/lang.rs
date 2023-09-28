use super::super::values::Language;
use crate::common::{VCardProperty, VCardValue};

pub struct LanguageProperty {
    languages: Vec<Language>,
}

impl LanguageProperty {
    pub fn new() -> Self {
        Self { languages: vec![] }
    }

    pub fn add(&mut self, language: Language) {
        self.languages.push(language);
    }
}

impl VCardProperty for LanguageProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for language in &self.languages {
            output.push_str(&language.format_value());
        }
        output
    }
}
