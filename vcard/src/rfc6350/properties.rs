/*
TODO:
-----------------
- PHOTO
- BDAY
- ANNIVERSARY
- GENDER
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
- URL
------------------
- FBURL
- CALADRURI
- CALURI
*/
pub mod full_name;
pub mod gender;
pub mod name;
pub mod nickname;
pub mod photo;

pub use full_name::FullNameProperty;
pub use gender::GenderProperty;
pub use name::NameProperty;
pub use nickname::NickNameProperty;
pub use photo::PhotoProperty;
