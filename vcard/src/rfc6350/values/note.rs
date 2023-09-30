use super::super::parameters::{LanguageParam, PrefParam, TypeParam, VCardType, ValueParam};
use crate::common::VCardValue;
use sp_vcard_derive::vcard_property_type;

#[vcard_property_type("NOTE")]
pub struct Note {
    value: ValueParam,
    type_param: TypeParam,
    lang_param: LanguageParam,
    pref_param: PrefParam,
}

impl Note {
    pub fn new() -> Self {
        Self {
            value: ValueParam::new(),
            type_param: TypeParam::new(),
            lang_param: LanguageParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn set_value(mut self, note: &str) -> Self {
        self.value.set(note);
        self
    }

    pub fn add_type(mut self, vc_type: VCardType) -> Self {
        let tp: TypeParam = self.type_param;
        self.type_param = tp.add(vc_type);
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

impl VCardValue for Note {
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
