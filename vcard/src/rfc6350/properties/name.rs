use super::super::values::Name;
use crate::common::{VCardProperty, VCardValue};

pub struct NameProperty {
    name: Name,
}

impl NameProperty {
    pub fn new() -> Self {
        NameProperty { name: Name::new() }
    }

    pub fn set(
        &mut self,
        names: &str,
        sur_names: &str,
        add_names: &str,
        prefixes: &str,
        suffixes: &str,
    ) {
        self.name
            .set(names, sur_names, add_names, prefixes, suffixes);
    }
}

impl VCardProperty for NameProperty {
    fn to_content(&self) -> String {
        self.name.format_value()
    }
}
