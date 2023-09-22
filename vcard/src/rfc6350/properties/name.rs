use crate::common::VCardProperty;

pub struct NameProperty {
    value: String,
}

impl NameProperty {
    pub fn new() -> Self {
        NameProperty { value: "".into() }
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

impl VCardProperty for NameProperty {
    fn to_content(&self) -> String {
        if self.value.len() == 0 {
            "".into()
        } else {
            format!("N:{}\n", &self.value)
        }
    }
}

pub struct FullNameProperty {
    full_names: Vec<String>,
}

impl FullNameProperty {
    pub fn new() -> Self {
        Self { full_names: vec![] }
    }

    pub fn add(&mut self, full_name: &str) {
        self.full_names.push(full_name.to_string());
    }
}

impl VCardProperty for FullNameProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for full_name in &self.full_names {
            let x = full_name.to_string();
            output.push_str(&format!("FN:{x}\n"));
        }

        if output.len() < 1 {
            panic!("FullName(FN) Required!")
        }

        output
    }
}

/// Noted
pub struct NickNameProperty {
    value: Vec<String>,
}

impl NickNameProperty {
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

impl VCardProperty for NickNameProperty {
    fn to_content(&self) -> String {
        if self.value.len() > 0 {
            let nicknames = self.value.clone().join(",");
            format!("NICKNAME:{nicknames}\n")
        } else {
            format!("")
        }
    }
}
