use crate::common::VCardValue;
use crate::rfc6350::parameters::{ArrayValueParam, LanguageParam};
use vcard_derive::vcard_property_type;

#[vcard_property_type("N")]
pub struct Name {
    family_names: ArrayValueParam,
    given_names: ArrayValueParam,
    additional_names: ArrayValueParam,
    honorific_prefixes: ArrayValueParam,
    honorific_suffixes: ArrayValueParam,
    lang_param: LanguageParam,
}

impl Name {
    pub fn new() -> Self {
        Self {
            family_names: ArrayValueParam::new(),
            given_names: ArrayValueParam::new(),
            additional_names: ArrayValueParam::new(),
            honorific_prefixes: ArrayValueParam::new(),
            honorific_suffixes: ArrayValueParam::new(),
            lang_param: LanguageParam::new(),
        }
    }

    /// the Family Names (also known as surnames)
    pub fn add_family_name(mut self, family_name: &str) -> Self {
        self.family_names.add(family_name);
        self
    }

    /// Just name
    pub fn add_given_name(mut self, given_name: &str) -> Self {
        self.given_names.add(given_name);
        self
    }

    /// also known as middle names
    ///
    /// maybe?
    pub fn add_additional_name(mut self, additional_name: &str) -> Self {
        self.additional_names.add(additional_name);
        self
    }

    /// e.g., Mr., Dr.
    pub fn add_honorific_prefix(mut self, honorific_prefix: &str) -> Self {
        self.honorific_prefixes.add(honorific_prefix);
        self
    }

    /// e.g., Jr.,M.D.,A.C.P.
    pub fn add_honorific_suffix(mut self, honorific_suffix: &str) -> Self {
        self.honorific_suffixes.add(honorific_suffix);
        self
    }

    pub fn set_language(mut self, lang: Option<&str>) -> Self {
        self.lang_param.set(lang);
        self
    }

    fn format_name(&self) -> String {
        format!(
            "{};{};{};{};{}",
            self.family_names,
            self.given_names,
            self.additional_names,
            self.honorific_prefixes,
            self.honorific_suffixes
        )
    }
}

impl VCardValue for Name {
    fn format_value(&self) -> String {
        let value = self.format_name();
        if value.len() < 5 {
            "".into()
        } else {
            format!("{}{}:{}\n", Self::get_value_type(), self.lang_param, value)
        }
    }
}
