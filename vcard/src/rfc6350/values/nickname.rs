use super::super::parameters::{LanguageParam, TypeParam, VCardType};
use crate::common::{VCardParam, VCardValue};
use vcard_derive::vcard_property_type;

#[vcard_property_type("NICKNAME")]
pub struct NickName {
    values: Vec<String>,
    type_param: TypeParam,
    lang_param: LanguageParam,
}

impl NickName {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            type_param: TypeParam::new(),
            lang_param: LanguageParam::new(),
        }
    }

    pub fn add_nickname(mut self, nickname: &str) -> Self {
        let trimmed = nickname.trim();
        if trimmed.len() > 0 {
            self.values.push(trimmed.into());
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

impl VCardValue for NickName {
    fn format_value(&self) -> String {
        if self.values.len() < 1 {
            "".into()
        } else {
            let nicknames = self.values.join(",");
            format!(
                "{}{}{}:{}\n",
                Self::get_value_type(),
                self.lang_param.format_param(),
                self.type_param.format_param(),
                nicknames
            )
        }
    }
}
