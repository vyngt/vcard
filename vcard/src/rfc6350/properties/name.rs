use crate::common::VCardProperty;

pub struct Name {
    value: String,
}

impl Name {
    pub fn new() -> Self {
        Name {
            value: String::from(";;;;"),
        }
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
        );
    }
}

impl VCardProperty for Name {
    fn to_content(&self) -> String {
        format!("N:{}\n", &self.value)
    }
}

pub struct FullName {
    value: String,
}

impl FullName {
    pub fn new() -> Self {
        Self {
            value: String::from(""),
        }
    }

    pub fn set(&mut self, full_name: &str) {
        self.value = format!("{}", full_name);
    }
}

impl VCardProperty for FullName {
    fn to_content(&self) -> String {
        format!("FN:{}\n", &self.value)
    }
}
