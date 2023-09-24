use super::super::values::FullName;
use crate::common::{VCardProperty, VCardValue};

pub struct FullNameProperty {
    full_names: Vec<FullName>,
}

impl FullNameProperty {
    pub fn new() -> Self {
        Self { full_names: vec![] }
    }

    pub fn push(&mut self, full_name: FullName) {
        self.full_names.push(full_name);
    }
}

impl VCardProperty for FullNameProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for full_name in &self.full_names {
            output.push_str(&full_name.format_value());
        }

        if output.len() < 1 {
            panic!("FullName(FN) Required!")
        }

        output
    }
}
