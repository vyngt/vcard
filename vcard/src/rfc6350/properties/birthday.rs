use super::super::values::Birthday;
use crate::common::{VCardProperty, VCardValue};

pub struct BirthdayProperty {
    birthday: Birthday,
}

impl BirthdayProperty {
    pub fn new() -> Self {
        Self {
            birthday: Birthday::new(),
        }
    }

    pub fn set(&mut self, year: i32, month: u32, day: u32) {
        self.birthday.set(year, month, day);
    }
}

impl VCardProperty for BirthdayProperty {
    fn to_content(&self) -> String {
        self.birthday.format_value()
    }
}
