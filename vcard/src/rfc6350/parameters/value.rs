use crate::common::VCardParam;
/// Just Text
pub struct ValueParam {
    value: String,
}

impl ValueParam {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn set(&mut self, value: &str) {
        let trimmed = value.trim();
        if trimmed.len() > 0 {
            self.value = trimmed.into();
        }
    }
}

impl VCardParam for ValueParam {
    fn format_param(&self) -> String {
        self.value.clone()
    }
}

impl std::fmt::Display for ValueParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_param())
    }
}

/// Multiple Text: repeated in COMMA-separated.
///
/// Only: N, NICKNAME, ADR, and CATEGORIES
pub struct ArrayValueParam {
    value: Vec<String>,
}

impl ArrayValueParam {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub fn add(&mut self, value: &str) {
        let trimmed = value.trim();
        if trimmed.len() > 0 {
            self.value.push(trimmed.into());
        }
    }
}

impl VCardParam for ArrayValueParam {
    fn format_param(&self) -> String {
        self.value.join(",").to_string()
    }
}

impl std::fmt::Display for ArrayValueParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_param())
    }
}
