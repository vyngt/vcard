use super::super::values::NickName;
use crate::common::{VCardProperty, VCardValue};

/// Noted
pub struct NickNameProperty {
    nicknames: Vec<NickName>,
}

impl NickNameProperty {
    pub fn new() -> Self {
        Self {
            nicknames: Vec::new(),
        }
    }

    pub fn push(&mut self, nickname: NickName) {
        self.nicknames.push(nickname);
    }
}

impl VCardProperty for NickNameProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for nickname in &self.nicknames {
            output.push_str(&nickname.format_value());
        }
        output
    }
}
