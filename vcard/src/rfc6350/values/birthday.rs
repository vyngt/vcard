use crate::common::VCardValue;
use crate::utils::DateTimeFormatter;
use sp_vcard_derive::vcard_property_type;

#[vcard_property_type("BDAY")]
pub struct Birthday {
    year: i32,
    month: u32,
    day: u32,
}

impl Birthday {
    pub fn new() -> Self {
        Self {
            year: 0,
            month: 0,
            day: 0,
        }
    }

    pub fn set(&mut self, year: i32, month: u32, day: u32) {
        if year == 0 || month == 0 || day == 0 {
            panic!("Invalid Date!")
        } else {
            self.year = year;
            self.month = month;
            self.day = day;
        }
    }
}

impl VCardValue for Birthday {
    fn format_value(&self) -> String {
        if self.year == 0 || self.month == 0 || self.day == 0 {
            "".into()
        } else {
            format!(
                "{}:{}\n",
                Self::get_value_type(),
                DateTimeFormatter::fmt_date(self.year, self.month, self.day)
                    .expect("Invalid Date!")
            )
        }
    }
}
