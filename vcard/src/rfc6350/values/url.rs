use super::super::parameters::{PrefParam, TypeParam, ValueParam};
use crate::common::VCardValue;
use sp_vcard_derive::{vcard_property_type, CommonTypeParamMixin};

#[vcard_property_type("URL")]
#[derive(CommonTypeParamMixin)]
pub struct URL {
    value: ValueParam,
    type_param: TypeParam,
    pref_param: PrefParam,
}

impl URL {
    pub fn new() -> Self {
        Self {
            value: ValueParam::new(),
            type_param: TypeParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn set_value(mut self, url: &str) -> Self {
        self.value.set(url);
        self
    }

    pub fn set_prefer(mut self, preferred: u8) -> Self {
        self.pref_param.set(preferred).unwrap();
        self
    }
}

impl VCardValue for URL {
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
