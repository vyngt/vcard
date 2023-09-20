pub mod properties;

use crate::common::VCardProperty;
use properties::{FullName, Name, NickName};

#[derive(vcard_derive::VCard)]
pub struct VCard40 {
    pub name: Name,
    pub full_name: FullName,
    pub nicknames: NickName,
}

impl VCard40 {
    fn version() -> &'static str {
        "4.0"
    }
}
