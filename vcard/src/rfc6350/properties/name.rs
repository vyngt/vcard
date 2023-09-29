use super::super::values::Name;
use crate::common::{VCardProperty, VCardValue};

pub struct NameProperty {
    name: Name,
}

impl NameProperty {
    pub fn new() -> Self {
        NameProperty { name: Name::new() }
    }

    pub fn set(&mut self, name: Name) {
        self.name = name;
    }
}

impl VCardProperty for NameProperty {
    fn to_content(&self) -> String {
        self.name.format_value()
    }
}
