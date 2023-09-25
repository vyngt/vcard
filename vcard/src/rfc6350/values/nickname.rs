use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

#[vcard_property_type("NICKNAME")]
pub struct NickName {
    value: String,
}

impl NickName {
    pub fn new(nickname: &str) -> Self {
        Self {
            value: nickname.into(),
        }
    }
}

impl VCardValue for NickName {
    fn format_value(&self) -> String {
        let trimmed = self.value.trim();
        if trimmed.len() > 0 {
            format!("{}:{}\n", Self::get_value_type(), trimmed)
        } else {
            "".into()
        }
    }
}
