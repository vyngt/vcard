pub mod parameters;
pub mod properties;
pub mod values;

use std::fs;
use std::path::Path;

use crate::common::VCardProperty;
use properties::{
    AddressProperty, AnniversaryProperty, BirthdayProperty, CategoryProperty, EmailProperty,
    FullNameProperty, GenderProperty, LanguageProperty, LogoProperty, NameProperty,
    NickNameProperty, NoteProperty, OrganizationProperty, PhotoProperty, RevProperty, RoleProperty,
    SoundProperty, SourceProperty, TelProperty, TitleProperty, URLProperty, XProperty,
};

#[derive(sp_vcard_derive::VCard)]
pub struct VCard40 {
    pub full_names: FullNameProperty,
    pub name: NameProperty,
    pub nicknames: NickNameProperty,
    pub photos: PhotoProperty,
    pub birthday: BirthdayProperty,
    pub anniversary: AnniversaryProperty,
    pub gender: GenderProperty,
    pub addresses: AddressProperty,
    pub tels: TelProperty,
    pub emails: EmailProperty,
    pub languages: LanguageProperty,
    pub titles: TitleProperty,
    pub roles: RoleProperty,
    pub logos: LogoProperty,
    pub orgs: OrganizationProperty,
    pub categories: CategoryProperty,
    pub notes: NoteProperty,
    pub rev: RevProperty,
    pub sounds: SoundProperty,
    pub urls: URLProperty,
    pub sources: SourceProperty,
    pub x_properties: XProperty,
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
