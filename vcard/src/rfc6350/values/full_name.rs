use super::super::parameters::{LanguageParam, TypeParam, VCardType};
use crate::common::{VCardParam, VCardValue};
use vcard_derive::vcard_property_type;

#[vcard_property_type("FN")]
pub struct FullName {
    value: String,
    type_param: TypeParam,
    lang_param: LanguageParam,
}

impl FullName {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            type_param: TypeParam::new(),
            lang_param: LanguageParam::new(),
        }
    }

    pub fn set_value(mut self, full_name: &str) -> Self {
        let trimmed = full_name.trim();
        if trimmed.len() > 0 {
            self.value = trimmed.into();
        }
        self
    }

    pub fn add_type(mut self, vc_type: VCardType) -> Self {
        self.type_param.push(vc_type);
        self
    }

    pub fn set_language(mut self, lang: Option<String>) -> Self {
        self.lang_param.set(lang);
        self
    }
}

impl VCardValue for FullName {
    fn format_value(&self) -> String {
        if self.value.len() > 0 {
            format!(
                "{}{}{}:{}\n",
                Self::get_value_type(),
                self.lang_param.format_param(),
                self.type_param.format_param(),
                self.value
            )
        } else {
            "".into()
        }
    }
}
