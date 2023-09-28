use super::super::values::Title;
use crate::common::{VCardProperty, VCardValue};

pub struct TitleProperty {
    titles: Vec<Title>,
}

impl TitleProperty {
    pub fn new() -> Self {
        Self { titles: vec![] }
    }

    pub fn add(&mut self, title: Title) {
        self.titles.push(title);
    }
}

impl VCardProperty for TitleProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for title in &self.titles {
            output.push_str(&title.format_value());
        }
        output
    }
}
