use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

#[vcard_property_type("FN")]
pub struct FullName {
    value: String,
}

impl FullName {
    pub fn new(full_name: &str) -> Self {
        FullName {
            value: full_name.into(),
        }
    }
}

impl VCardValue for FullName {
    fn format_value(&self) -> String {
        let trimmed = self.value.trim();
        if trimmed.len() > 0 {
            format!("{}:{}\n", Self::get_value_type(), trimmed)
        } else {
            "".into()
        }
    }
}
