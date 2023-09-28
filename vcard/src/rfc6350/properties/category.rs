use super::super::values::Category;
use crate::common::{VCardProperty, VCardValue};

pub struct CategoryProperty {
    categories: Vec<Category>,
}

impl CategoryProperty {
    pub fn new() -> Self {
        Self { categories: vec![] }
    }

    pub fn add(&mut self, category: Category) {
        self.categories.push(category);
    }
}

impl VCardProperty for CategoryProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for category in &self.categories {
            output.push_str(&category.format_value());
        }
        output
    }
}
