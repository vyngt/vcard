pub mod parameters;
pub mod properties;
pub mod values;

use std::fs;
use std::path::Path;

use crate::common::VCardProperty;
use properties::{
    AnniversaryProperty, BirthdayProperty, CategoryProperty, EmailProperty, FullNameProperty,
    GenderProperty, LanguageProperty, NameProperty, NickNameProperty, PhotoProperty, RevProperty,
    RoleProperty, TelProperty, TitleProperty, URLProperty,
};

#[derive(vcard_derive::VCard)]
pub struct VCard40 {
    pub full_names: FullNameProperty,
    pub name: NameProperty,
    pub nicknames: NickNameProperty,
    pub photos: PhotoProperty,
    pub birthday: BirthdayProperty,
    pub anniversary: AnniversaryProperty,
    pub gender: GenderProperty,
    // ADR
    pub tel: TelProperty,
    pub email: EmailProperty,
    // IMP
    pub language: LanguageProperty,
    pub title: TitleProperty,
    pub role: RoleProperty,
    pub categories: CategoryProperty,
    pub urls: URLProperty,
    pub rev: RevProperty,
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

impl std::fmt::Display for VCard40 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.generate_vcard())
    }
}
