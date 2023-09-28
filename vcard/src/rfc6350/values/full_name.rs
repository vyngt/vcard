use super::super::parameters::{LanguageParam, PrefParam, TypeParam, VCardType, ValueParam};
use crate::common::{VCardParam, VCardValue};
use vcard_derive::vcard_property_type;

#[vcard_property_type("FN")]
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

    pub fn add_type(mut self, vc_type: VCardType) -> Self {
        let tp: TypeParam = self.type_param;
        self.type_param = tp.add(vc_type);
        self
    }

    pub fn set_language(mut self, lang: Option<String>) -> Self {
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
        let value = self.value.format_param();
        if value.len() > 0 {
            format!(
                "{}{}{}{}:{}\n",
                Self::get_value_type(),
                self.pref_param.format_param(),
                self.lang_param.format_param(),
                self.type_param.format_param(),
                value
            )
        } else {
            "".into()
        }
    }
}
