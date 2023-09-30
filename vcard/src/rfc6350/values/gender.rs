use crate::common::VCardValue;
use sp_vcard_derive::vcard_property_type;

pub enum IGender {
    None,
    Male,
    Female,
    Other,
    NotApplicable,
    Unknown,
}

#[vcard_property_type("GENDER")]
pub struct Gender {
    gender: IGender,
}

impl Gender {
    pub fn new() -> Self {
        Gender {
            gender: IGender::None,
        }
    }

    pub fn set(&mut self, gender: IGender) {
        self.gender = gender;
    }
}

impl VCardValue for Gender {
    fn format_value(&self) -> String {
        let data: String = match self.gender {
            IGender::None => "".into(),
            IGender::Male => "M".into(),
            IGender::Female => "F".into(),
            IGender::Other => "O".into(),
            IGender::NotApplicable => "N".into(),
            IGender::Unknown => "U".into(),
        };

        if data.len() > 0 {
            format!("{}:{}\n", Self::get_value_type(), data)
        } else {
            "".into()
        }
    }
}
