use crate::common::VCardParam;
use vcard_derive::vcard_property_type;

#[vcard_property_type("PREF")]
pub struct PrefParam {
    preferred: Option<u8>,
}

impl PrefParam {
    pub fn new() -> Self {
        Self { preferred: None }
    }

    pub fn set(&mut self, preferred: u8) -> Result<(), &'static str> {
        if 0 < preferred && preferred < 101 {
            self.preferred = Some(preferred);
            Ok(())
        } else {
            Err("Prefer value condition(x): 1 <= x <= 100")
        }
    }
}

impl VCardParam for PrefParam {
    fn format_param(&self) -> String {
        match self.preferred {
            Some(p) => format!(";{}={}", Self::get_value_type(), p.to_string()),
            None => "".into(),
        }
    }
}

impl std::fmt::Display for PrefParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_param())
    }
}
