use super::super::values::Rev;
use crate::common::{VCardProperty, VCardValue};

pub struct RevProperty {
    rev: Rev,
}

impl RevProperty {
    pub fn new() -> Self {
        Self { rev: Rev::new() }
    }

    pub fn update_current(&mut self) {
        self.rev.update();
    }
}

impl VCardProperty for RevProperty {
    fn to_content(&self) -> String {
        self.rev.format_value()
    }
}
