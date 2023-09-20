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

/// Noted
pub struct NickName {
    value: Vec<String>,
}

impl NickName {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub fn set(&mut self, nicknames: Vec<&str>) {
        self.value.clear();
        for nickname in nicknames {
            self.value.push(nickname.to_string());
        }
    }
}

impl VCardProperty for NickName {
    fn to_content(&self) -> String {
        if self.value.len() > 0 {
            let nicknames = self.value.clone().join(",");
            format!("NICKNAME:{nicknames}\n")
        } else {
            format!("")
        }
    }
}
