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

/// vCard 4.0
/// # Examples
///
/// ```
/// use sp_vcard::rfc6350::parameters::BaseType;
/// use sp_vcard::rfc6350::values::{
///     Category, Email, FullName, IGender, Language, NickName, Role, Title, URL,
/// };
/// use sp_vcard::rfc6350::VCard40;
///
/// let mut vc = VCard40::new();
/// vc.full_names.add(
///     FullName::new()
///         .set_value("Nguyen The Vy")
///         .set_language(Some("vi".into()))
///         .add_base_type(BaseType::HOME)
///         .add_base_type(BaseType::WORK),
/// );
/// vc.gender.set(IGender::Male);
/// vc.emails.add(
///     Email::new()
///         .set_value("vyngt@outlook.com")
///         .set_prefer(1)
///         .add_base_type(BaseType::WORK),
/// );
/// vc.emails.add(
///     Email::new()
///         .set_value("ntvy2k@gmail.com")
///         .set_prefer(2)
///         .add_base_type(BaseType::HOME),
/// );
/// vc.languages
///     .add(Language::new().set_prefer(1).set_value("vi"));
/// vc.categories.add(
///     Category::new()
///         .add_category("Rust")
///         .add_category("Python")
///         .add_category("Javascript"),
/// );
/// vc.nicknames
///     .add(NickName::new().add_nickname("TheVy").add_nickname("VyNT"));
/// vc.urls.add(
///     URL::new()
///         .set_value("https://github.com/vyngt")
///         .add_x_type("Github"),
/// );
/// vc.titles.add(
///     Title::new()
///         .set_value("Rust Developer")
///         .add_base_type(BaseType::HOME),
/// );
/// vc.titles.add(
///     Title::new()
///         .set_value("Python Developer")
///         .add_base_type(BaseType::WORK),
/// );
/// vc.roles.add(Role::new().set_value("Story Teller"));
/// let expected = "BEGIN:VCARD\n\
/// VERSION:4.0\n\
/// FN;LANGUAGE=vi;TYPE=HOME,WORK:Nguyen The Vy\n\
/// NICKNAME:TheVy,VyNT\n\
/// GENDER:M\n\
/// EMAIL;PREF=1;TYPE=WORK:vyngt@outlook.com\n\
/// EMAIL;PREF=2;TYPE=HOME:ntvy2k@gmail.com\n\
/// LANG;PREF=1:vi\n\
/// TITLE;TYPE=HOME:Rust Developer\n\
/// TITLE;TYPE=WORK:Python Developer\n\
/// ROLE:Story Teller\n\
/// CATEGORIES:Rust,Python,Javascript\n\
/// URL;TYPE=GITHUB:https://github.com/vyngt\n\
/// END:VCARD";
/// assert_eq!(vc.to_string(), expected);
/// ```
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

    /// Write vCard to file
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
