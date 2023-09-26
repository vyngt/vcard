use super::super::values::Anniversary;
use crate::common::{VCardProperty, VCardValue};

pub struct AnniversaryProperty {
    anniversary: Anniversary,
}

impl AnniversaryProperty {
    pub fn new() -> Self {
        Self {
            anniversary: Anniversary::new(),
        }
    }

    pub fn set(&mut self, year: i32, month: u32, day: u32) {
        self.anniversary.set(year, month, day);
    }
}

impl VCardProperty for AnniversaryProperty {
    fn to_content(&self) -> String {
        self.anniversary.format_value()
    }
}
