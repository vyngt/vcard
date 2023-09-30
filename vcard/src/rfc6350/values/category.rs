use super::super::parameters::{ArrayValueParam, PrefParam, TypeParam, VCardType};
use crate::common::VCardValue;
use sp_vcard_derive::vcard_property_type;

#[vcard_property_type("CATEGORIES")]
pub struct Category {
    value: ArrayValueParam,
    type_param: TypeParam,
    pref_param: PrefParam,
}

impl Category {
    pub fn new() -> Self {
        Self {
            value: ArrayValueParam::new(),
            type_param: TypeParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn add_category(mut self, category: &str) -> Self {
        self.value.add(category);
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

impl VCardValue for Category {
    fn format_value(&self) -> String {
        let value = self.value.to_string();
        if value.len() > 0 {
            format!(
                "{}{}{}:{}\n",
                Self::get_value_type(),
                self.pref_param,
                self.type_param,
                value
            )
        } else {
            "".into()
        }
    }
}
