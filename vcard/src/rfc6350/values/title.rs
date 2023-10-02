use super::super::parameters::{LanguageParam, PrefParam, TypeParam, ValueParam};
use crate::common::VCardValue;
use sp_vcard_derive::{vcard_property_type, CommonTypeParamMixin};

#[vcard_property_type("TITLE")]
#[derive(CommonTypeParamMixin)]
pub struct Title {
    value: ValueParam,
    lang_param: LanguageParam,
    type_param: TypeParam,
    pref_param: PrefParam,
}

impl Title {
    pub fn new() -> Self {
        Self {
            value: ValueParam::new(),
            lang_param: LanguageParam::new(),
            type_param: TypeParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn set_value(mut self, title: &str) -> Self {
        self.value.set(title);
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

impl VCardValue for Title {
    fn format_value(&self) -> String {
        let value = self.value.to_string();
        if value.len() > 0 {
            format!(
                "{}{}{}{}:{}\n",
                Self::get_value_type(),
                self.lang_param,
                self.pref_param,
                self.type_param,
                value
            )
        } else {
            "".into()
        }
    }
}
