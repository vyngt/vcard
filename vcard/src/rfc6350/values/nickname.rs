use super::super::parameters::{ArrayValueParam, LanguageParam, PrefParam, TypeParam};
use crate::common::VCardValue;
use sp_vcard_derive::{vcard_property_type, CommonTypeParamMixin};

#[vcard_property_type("NICKNAME")]
#[derive(CommonTypeParamMixin)]
pub struct NickName {
    value: ArrayValueParam,
    type_param: TypeParam,
    lang_param: LanguageParam,
    pref_param: PrefParam,
}

impl NickName {
    pub fn new() -> Self {
        Self {
            value: ArrayValueParam::new(),
            type_param: TypeParam::new(),
            lang_param: LanguageParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn add_nickname(mut self, nickname: &str) -> Self {
        self.value.add(nickname);
        self
    }

    pub fn set_language(mut self, lang: Option<&str>) -> Self {
        self.lang_param.set(lang);
        self
    }

    pub fn set_prefer(mut self, preferred: u8) -> Self {
        self.pref_param.set(preferred).unwrap();
        self
    }
}

impl VCardValue for NickName {
    fn format_value(&self) -> String {
        let value = self.value.to_string();
        if value.len() < 1 {
            "".into()
        } else {
            format!(
                "{}{}{}{}:{}\n",
                Self::get_value_type(),
                self.pref_param,
                self.lang_param,
                self.type_param,
                value,
            )
        }
    }
}
