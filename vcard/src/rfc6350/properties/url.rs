use super::super::values::URL;
use crate::common::{VCardProperty, VCardValue};

pub struct URLProperty {
    urls: Vec<URL>,
}

impl URLProperty {
    pub fn new() -> Self {
        Self { urls: vec![] }
    }

    pub fn push(&mut self, url: URL) {
        self.urls.push(url);
    }
}

impl VCardProperty for URLProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for url in &self.urls {
            output.push_str(&url.format_value());
        }
        output
    }
}
