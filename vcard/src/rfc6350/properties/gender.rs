use crate::common::VCardProperty;

pub enum IGender {
    Male,
    Female,
    Other,
    Unknown,
    NotFill,
}

pub struct Gender {
    value: String,
}

impl Gender {
    pub fn new() -> Self {
        Gender { value: "".into() }
    }

    pub fn set(&mut self, gender: IGender) {
        self.value = match gender {
            IGender::Male => "M".into(),
            IGender::Female => "F".into(),
            IGender::Other => "O".into(),
            IGender::Unknown => "U".into(),
            IGender::NotFill => "".into(),
        }
    }
}

impl VCardProperty for Gender {
    fn to_content(&self) -> String {
        if self.value.len() > 0 {
            format!("GENDER:{}\n", &self.value)
        } else {
            "".into()
        }
    }
}
