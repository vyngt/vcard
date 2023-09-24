use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

#[vcard_property_type("FN")]
pub struct FullName {
    full_name: String,
}

impl FullName {
    pub fn new(full_name: &str) -> Self {
        FullName {
            full_name: full_name.into(),
        }
    }
}

impl VCardValue for FullName {
    fn format_value(&self) -> String {
        let trimmed = self.full_name.trim();
        if trimmed.len() > 0 {
            format!("{}:{}\n", Self::get_value_type(), trimmed)
        } else {
            "".into()
        }
    }
}
