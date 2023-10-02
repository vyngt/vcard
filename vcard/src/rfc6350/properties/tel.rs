use super::super::values::Tel;
use crate::common::{VCardProperty, VCardValue};

/// # Tel
/// To specify the telephone number for telephony communication
/// with the object the vCard represents.
pub struct TelProperty {
    tels: Vec<Tel>,
}

impl TelProperty {
    pub fn new() -> Self {
        Self { tels: vec![] }
    }

    pub fn add(&mut self, tel: Tel) {
        self.tels.push(tel);
    }
}

impl VCardProperty for TelProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for tel in &self.tels {
            output.push_str(&tel.format_value());
        }
        output
    }
}
