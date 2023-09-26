use super::super::parameters::{TypeParam, VCardType};
use crate::common::{VCardParam, VCardValue};
use vcard_derive::vcard_property_type;

#[vcard_property_type("URL")]
pub struct URL {
    value: String,
    type_param: TypeParam,
}

impl URL {
    pub fn new() -> Self {
        Self {
            value: "".into(),
            type_param: TypeParam::new(),
        }
    }

    pub fn set_value(mut self, url: &str) -> Self {
        self.value = url.into();
        self
    }

    pub fn add_type(mut self, vc_type: VCardType) -> Self {
        let tp: TypeParam = self.type_param;
        self.type_param = tp.add(vc_type);
        self
    }
}

impl VCardValue for URL {
    fn format_value(&self) -> String {
        let trimmed = self.value.trim();
        if trimmed.len() > 0 {
            format!(
                "{}{}:{}\n",
                Self::get_value_type(),
                self.type_param.format_param(),
                trimmed
            )
        } else {
            "".into()
        }
    }
}
