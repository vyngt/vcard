/*
TODO:
-----------------
- PHOTO
- ADR
- TEL
- EMAIL
- IMPP
- LANG
- TZ
-----------------
- GEO
-----------------
- TITLE
- ROLE
- LOGO
- ORG
- MEMBER
- RELATED
------------------
- CATEGORIES
- NOTE
- PRODID
- REV
- SOUND
- UID
- CLIENTPIDMAP
------------------
- FBURL
- CALADRURI
- CALURI
*/
pub mod anniversary;
pub mod birthday;
pub mod email;
pub mod full_name;
pub mod gender;
pub mod name;
pub mod nickname;
pub mod photo;
pub mod rev;
pub mod url;

pub use anniversary::AnniversaryProperty;
pub use birthday::BirthdayProperty;
pub use email::EmailProperty;
pub use full_name::FullNameProperty;
pub use gender::GenderProperty;
pub use name::NameProperty;
pub use nickname::NickNameProperty;
pub use photo::PhotoProperty;
pub use rev::RevProperty;
pub use url::URLProperty;
