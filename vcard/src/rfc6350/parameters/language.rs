use crate::common::VCardParam;
use vcard_derive::vcard_property_type;

#[vcard_property_type("LANGUAGE")]
pub struct LanguageParam {
    value: Option<String>,
}

impl LanguageParam {
    pub fn new() -> Self {
        Self { value: None }
    }

    /// TODO: Validate
    pub fn set(&mut self, lang: Option<String>) {
        self.value = lang;
    }
}

impl VCardParam for LanguageParam {
    fn format_param(&self) -> String {
        match &self.value {
            Some(l) => format!(";{}={}", Self::get_value_type(), l),
            None => "".into(),
        }
    }
}

impl std::fmt::Display for LanguageParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_param())
    }
}
