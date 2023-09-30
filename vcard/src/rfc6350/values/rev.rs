use crate::common::VCardValue;
use sp_vcard_derive::vcard_property_type;

use chrono::prelude::*;

#[vcard_property_type("REV")]
pub struct Rev {
    value: Option<DateTime<Utc>>,
}

impl Rev {
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn update(&mut self) {
        self.value = Some(Utc::now());
    }
}

impl VCardValue for Rev {
    fn format_value(&self) -> String {
        match self.value {
            Some(dt) => format!(
                "{}:{}\n",
                Self::get_value_type(),
                dt.format("%Y%m%dT%H%M%S")
            ),
            None => "".into(),
        }
    }
}
