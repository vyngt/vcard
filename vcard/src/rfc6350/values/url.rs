use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

#[vcard_property_type("URL")]
pub struct URL {
    url: String,
}

impl URL {
    pub fn new(url: &str) -> Self {
        URL { url: url.into() }
    }
}

impl VCardValue for URL {
    fn format_value(&self) -> String {
        let trimmed = self.url.trim();
        if trimmed.len() > 0 {
            format!("{}:{}\n", Self::get_value_type(), trimmed)
        } else {
            "".into()
        }
    }
}
