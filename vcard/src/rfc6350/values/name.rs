use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

// TODO: Rewrite this
#[vcard_property_type("N")]
pub struct Name {
    value: String,
}

impl Name {
    pub fn new() -> Self {
        Name { value: "".into() }
    }

    pub fn set(
        &mut self,
        names: &str,
        sur_names: &str,
        add_names: &str,
        prefixes: &str,
        suffixes: &str,
    ) {
        self.value = format!(
            "{};{};{};{};{}",
            sur_names, names, add_names, prefixes, suffixes
        )
    }
}

impl VCardValue for Name {
    fn format_value(&self) -> String {
        if self.value.len() == 0 {
            "".into()
        } else {
            format!("{}:{}\n", Self::get_value_type(), self.value)
        }
    }
}
