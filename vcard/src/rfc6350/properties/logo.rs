use super::super::values::Logo;
use crate::common::{VCardProperty, VCardValue};

pub struct LogoProperty {
    logos: Vec<Logo>,
}

impl LogoProperty {
    pub fn new() -> Self {
        Self { logos: vec![] }
    }

    pub fn add(&mut self, logo: Logo) {
        self.logos.push(logo);
    }
}

impl VCardProperty for LogoProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for logo in &self.logos {
            output.push_str(&logo.format_value());
        }

        output
    }
}
