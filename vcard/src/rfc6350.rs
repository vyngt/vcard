pub mod parameters;
pub mod properties;
pub mod values;

use std::fs;
use std::path::Path;

use crate::common::VCardProperty;
use properties::{
    BirthdayProperty, FullNameProperty, GenderProperty, NameProperty, NickNameProperty,
    PhotoProperty, URLProperty,
};

#[derive(vcard_derive::VCard)]
pub struct VCard40 {
    pub full_names: FullNameProperty,
    pub name: NameProperty,
    pub nicknames: NickNameProperty,
    pub gender: GenderProperty,
    pub birthday: BirthdayProperty,
    pub photos: PhotoProperty,
    pub urls: URLProperty,
}

impl VCard40 {
    pub fn version() -> &'static str {
        "4.0"
    }

    pub fn write_to_file(&self, pathname: &str) -> bool {
        let file = Path::new(pathname);
        let ok = fs::write(file, self.generate_vcard());
        match ok {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
