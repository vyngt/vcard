use super::super::parameters::{PrefParam, TypeParam, VCardType, ValueParam};
use crate::common::VCardValue;
use sp_vcard_derive::vcard_property_type;

// TODO: Validate

#[vcard_property_type("EMAIL")]
pub struct Email {
    value: ValueParam,
    type_param: TypeParam,
    pref_param: PrefParam,
}

impl Email {
    pub fn new() -> Self {
        Self {
            value: ValueParam::new(),
            type_param: TypeParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn set_value(mut self, email: &str) -> Self {
        self.value.set(email);
        self
    }

    pub fn add_type(mut self, vc_type: VCardType) -> Self {
        match vc_type {
            VCardType::Tel(_) => self,
            other => {
                let tp: TypeParam = self.type_param;
                self.type_param = tp.add(other);
                self
            }
        }
    }

    pub fn set_prefer(mut self, preferred: u8) -> Self {
        self.pref_param.set(preferred).unwrap();
        self
    }
}

impl VCardValue for Email {
    fn format_value(&self) -> String {
        let value = self.value.to_string();
        if value.len() > 0 {
            format!(
                "{}{}{}:{}\n",
                Self::get_value_type(),
                self.pref_param,
                self.type_param,
                value
            )
        } else {
            "".into()
        }
    }
}
