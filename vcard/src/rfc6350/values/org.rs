use super::super::parameters::{LanguageParam, PrefParam, TypeParam, VCardType, ValueParam};
use crate::common::{VCardParam, VCardValue};
use sp_vcard_derive::vcard_property_type;

#[vcard_property_type("ORG")]
pub struct Organization {
    value: ValueParam,
    ou: Vec<String>,
    type_param: TypeParam,
    lang_param: LanguageParam,
    pref_param: PrefParam,
}

impl Organization {
    pub fn new() -> Self {
        Self {
            value: ValueParam::new(),
            ou: Vec::new(),
            type_param: TypeParam::new(),
            lang_param: LanguageParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    pub fn set_value(mut self, organization: &str) -> Self {
        self.value.set(organization);
        self
    }

    /// Add organization unit
    ///
    pub fn add_ou(mut self, organization_unit: &str) -> Self {
        let trimmed = organization_unit.trim();
        if trimmed.len() > 0 {
            self.ou.push(trimmed.into());
        }
        self
    }

    fn format_org_value(&self) -> String {
        let value_str = self.value.format_param();
        if value_str.len() < 1 {
            return "".into();
        }
        let ou_str = self.ou.join(";");
        if ou_str.len() > 0 {
            format!("{};{}", value_str, ou_str)
        } else {
            value_str
        }
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

impl VCardValue for Organization {
    fn format_value(&self) -> String {
        let value = self.format_org_value();
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
