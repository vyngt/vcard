/*
TODO:
-----------------
- IMPP
- TZ
-----------------
- GEO
-----------------
- MEMBER
- RELATED
------------------
- PRODID
- UID
- CLIENTPIDMAP
------------------
- FBURL
- CALADRURI
- CALURI
*/
pub mod adr;
pub mod anniversary;
pub mod birthday;
pub mod category;
pub mod email;
pub mod full_name;
pub mod gender;
pub mod lang;
pub mod logo;
pub mod name;
pub mod nickname;
pub mod note;
pub mod org;
pub mod photo;
pub mod rev;
pub mod role;
pub mod sound;
pub mod tel;
pub mod title;
pub mod url;

pub use adr::AddressProperty;
pub use anniversary::AnniversaryProperty;
pub use birthday::BirthdayProperty;
pub use category::CategoryProperty;
pub use email::EmailProperty;
pub use full_name::FullNameProperty;
pub use gender::GenderProperty;
pub use lang::LanguageProperty;
pub use logo::LogoProperty;
pub use name::NameProperty;
pub use nickname::NickNameProperty;
pub use note::NoteProperty;
pub use org::OrganizationProperty;
pub use photo::PhotoProperty;
pub use rev::RevProperty;
pub use role::RoleProperty;
pub use sound::SoundProperty;
pub use tel::TelProperty;
pub use title::TitleProperty;
pub use url::URLProperty;
