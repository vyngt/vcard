use crate::common::VCardProperty;

pub enum IGender {
    None,
    Male,
    Female,
    Other,
    NotApplicable,
    Unknown,
}

pub struct GenderProperty {
    value: String,
}

impl GenderProperty {
    pub fn new() -> Self {
        GenderProperty { value: "".into() }
    }

    pub fn set(&mut self, gender: IGender) {
        self.value = match gender {
            IGender::None => "".into(),
            IGender::Male => "M".into(),
            IGender::Female => "F".into(),
            IGender::Other => "O".into(),
            IGender::NotApplicable => "N".into(),
            IGender::Unknown => "U".into(),
        }
    }
}

impl VCardProperty for GenderProperty {
    fn to_content(&self) -> String {
        if self.value.len() > 0 {
            format!("GENDER:{}\n", &self.value)
        } else {
            "".into()
        }
    }
}
