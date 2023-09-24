use super::super::values::URL;
use crate::common::{VCardProperty, VCardValue};

pub struct URLProperty {
    urls: Vec<URL>,
}

impl URLProperty {
    pub fn new() -> Self {
        Self { urls: vec![] }
    }

    pub fn push_url(&mut self, url: &str) {
        self.urls.push(URL::new(url));
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
