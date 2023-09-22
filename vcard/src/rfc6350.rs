pub mod properties;

use std::fs;
use std::path::Path;

use crate::common::VCardProperty;
use properties::{FullName, Gender, Name, NickName};

#[derive(vcard_derive::VCard)]
pub struct VCard40 {
    pub full_names: FullName,
    pub name: Name,
    pub nicknames: NickName,
    pub gender: Gender,
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
