use super::super::parameters::{PrefParam, ValueParam};
use crate::common::VCardValue;
use sp_vcard_derive::vcard_property_type;

#[vcard_property_type("SOURCE")]
pub struct Source {
    value: ValueParam,
    pref_param: PrefParam,
}

impl Source {
    pub fn new() -> Self {
        Self {
            value: ValueParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn set_value(mut self, source: &str) -> Self {
        self.value.set(source);
        self
    }

    pub fn set_prefer(mut self, preferred: u8) -> Self {
        self.pref_param.set(preferred).unwrap();
        self
    }
}

impl VCardValue for Source {
    fn format_value(&self) -> String {
        let value = self.value.to_string();
        if value.len() > 0 {
            format!("{}{}:{}\n", Self::get_value_type(), self.pref_param, value)
        } else {
            "".into()
        }
    }
}
