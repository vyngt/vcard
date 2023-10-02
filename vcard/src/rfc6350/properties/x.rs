use super::super::values::X;
use crate::common::{VCardProperty, VCardValue};

pub struct XProperty {
    xs: Vec<X>,
}

impl XProperty {
    pub fn new() -> Self {
        Self { xs: vec![] }
    }

    pub fn add(&mut self, x: X) {
        self.xs.push(x);
    }
}

impl VCardProperty for XProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for x in &self.xs {
            output.push_str(&x.format_value());
        }
        output
    }
}
