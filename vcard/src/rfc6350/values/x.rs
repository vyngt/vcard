use super::super::parameters::{LanguageParam, PrefParam, TypeParam, ValueParam};
use crate::common::VCardValue;
use sp_vcard_derive::{vcard_property_type, CommonTypeParamMixin};

#[vcard_property_type("X")]
#[derive(CommonTypeParamMixin)]
pub struct X {
    property_name: ValueParam,
    value: ValueParam,
    lang_param: LanguageParam,
    type_param: TypeParam,
    pref_param: PrefParam,
}

impl X {
    pub fn new() -> Self {
        Self {
            property_name: ValueParam::new(),
            value: ValueParam::new(),
            lang_param: LanguageParam::new(),
            type_param: TypeParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn property_name(mut self, property_name: &str) -> Self {
        let split_iter = property_name.split_ascii_whitespace();
        let mut out = String::new();
        for s in split_iter {
            out.push_str(s);
        }
        self.property_name.set(&out);
        self
    }

    pub fn value(mut self, value: &str) -> Self {
        self.value.set(value);
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

impl VCardValue for X {
    fn format_value(&self) -> String {
        let value = self.value.to_string();
        let property_name = self.property_name.to_string().to_uppercase();
        if value.len() > 0 && property_name.len() > 0 {
            format!(
                "{}-{}{}{}{}:{}\n",
                Self::get_value_type(),
                property_name,
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
