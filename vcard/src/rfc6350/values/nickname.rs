use super::super::parameters::{VCardType, VCardTypeParams};
use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

#[vcard_property_type("NICKNAME")]
pub struct NickName {
    values: Vec<String>,
    types: VCardTypeParams,
}

impl NickName {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            types: VCardTypeParams::new(),
        }
    }

    pub fn add_nickname(mut self, nickname: &str) -> NickName {
        let trimmed = nickname.trim();
        if trimmed.len() > 0 {
            self.values.push(trimmed.into());
        }
        self
    }

    pub fn add_type(mut self, vc_type: VCardType) -> NickName {
        self.types.push(vc_type);
        self
    }
}

impl VCardValue for NickName {
    fn format_value(&self) -> String {
        if self.values.len() < 1 {
            "".into()
        } else {
            let nicknames = self.values.join(",");
            let types = self.types.to_value();
            format!("{}{}:{}\n", Self::get_value_type(), types, nicknames)
        }
    }
}
