use super::super::values::Email;
use crate::common::{VCardProperty, VCardValue};

pub struct EmailProperty {
    emails: Vec<Email>,
}

impl EmailProperty {
    pub fn new() -> Self {
        Self { emails: vec![] }
    }

    pub fn add(&mut self, email: Email) {
        self.emails.push(email);
    }
}

impl VCardProperty for EmailProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for email in &self.emails {
            output.push_str(&email.format_value());
        }
        output
    }
}
