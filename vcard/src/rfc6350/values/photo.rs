use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

#[vcard_property_type("PHOTO")]
pub struct Photo {
    value: String,
}

impl Photo {
    pub fn from_url(url: &str) -> Self {
        Photo { value: url.into() }
    }

    /// TODO
    pub fn from_base64(base64_string: &str) -> Self {
        // Process...
        Photo {
            value: base64_string.into(),
        }
    }
}

impl VCardValue for Photo {
    fn format_value(&self) -> String {
        format!("{}:{}\n", Self::get_value_type(), &self.value)
    }
}
