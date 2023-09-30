use crate::common::VCardValue;
use crate::rfc6350::parameters::media::ImageType;
use crate::rfc6350::parameters::ValueOrImageParam;
use vcard_derive::vcard_property_type;

#[vcard_property_type("PHOTO")]
pub struct Photo {
    value: ValueOrImageParam,
}

impl Photo {
    pub fn new() -> Self {
        Self {
            value: ValueOrImageParam::new(),
        }
    }

    pub fn set_uri(mut self, uri: &str) -> Self {
        self.value.set_uri(uri);
        self
    }

    pub fn set_bytes_data(mut self, data: Vec<u8>, datatype: ImageType) -> Self {
        self.value.set_bytes_data(data, datatype);
        self
    }
}

impl VCardValue for Photo {
    fn format_value(&self) -> String {
        let value = self.value.to_string();
        if value.len() > 0 {
            format!("{}:{}\n", Self::get_value_type(), self.value)
        } else {
            "".into()
        }
    }
}
