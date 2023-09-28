use super::super::parameters::{PrefParam, TypeParam, VCardType, ValueParam};
use crate::common::{VCardParam, VCardValue};
use vcard_derive::vcard_property_type;

#[vcard_property_type("URL")]
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

    pub fn add_type(mut self, vc_type: VCardType) -> Self {
        let tp: TypeParam = self.type_param;
        self.type_param = tp.add(vc_type);
        self
    }

    pub fn set_prefer(mut self, preferred: u8) -> Self {
        self.pref_param.set(preferred).unwrap();
        self
    }
}

impl VCardValue for URL {
    fn format_value(&self) -> String {
        let value = self.value.format_param();
        if value.len() > 0 {
            format!(
                "{}{}{}:{}\n",
                Self::get_value_type(),
                self.pref_param.format_param(),
                self.type_param.format_param(),
                value
            )
        } else {
            "".into()
        }
    }
}
