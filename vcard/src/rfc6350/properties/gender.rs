use super::super::values::{Gender, IGender};
use crate::common::{VCardProperty, VCardValue};

pub struct GenderProperty {
    gender: Gender,
}

impl GenderProperty {
    pub fn new() -> Self {
        GenderProperty {
            gender: Gender::new(),
        }
    }

    pub fn set(&mut self, gender: IGender) {
        self.gender.set(gender)
    }
}

impl VCardProperty for GenderProperty {
    fn to_content(&self) -> String {
        self.gender.format_value()
    }
}
