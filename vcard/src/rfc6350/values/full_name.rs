use super::super::parameters::{LanguageParam, PrefParam, TypeParam, ValueParam};
use crate::common::VCardValue;
use sp_vcard_derive::{vcard_property_type, CommonTypeParamMixin};

#[vcard_property_type("FN")]
#[derive(CommonTypeParamMixin)]
pub struct FullName {
    value: ValueParam,
    type_param: TypeParam,
    lang_param: LanguageParam,
    pref_param: PrefParam,
}

impl FullName {
    pub fn new() -> Self {
        Self {
            value: ValueParam::new(),
            type_param: TypeParam::new(),
            lang_param: LanguageParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn set_value(mut self, full_name: &str) -> Self {
        self.value.set(full_name);
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

impl VCardValue for FullName {
    fn format_value(&self) -> String {
        let value = self.value.to_string();
        if value.len() > 0 {
            format!(
                "{}{}{}{}:{}\n",
                Self::get_value_type(),
                self.pref_param,
                self.lang_param,
                self.type_param,
                value
            )
        } else {
            "".into()
        }
    }
}
