/*
TODO:
-----------------
- PHOTO
- ADR
- TEL
- IMPP
- TZ
-----------------
- GEO
-----------------
- LOGO
- ORG
- MEMBER
- RELATED
------------------
- NOTE
- PRODID
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
pub mod category;
pub mod email;
pub mod full_name;
pub mod gender;
pub mod lang;
pub mod name;
pub mod nickname;
pub mod photo;
pub mod rev;
pub mod role;
pub mod tel;
pub mod title;
pub mod url;

pub use anniversary::AnniversaryProperty;
pub use birthday::BirthdayProperty;
pub use category::CategoryProperty;
pub use email::EmailProperty;
pub use full_name::FullNameProperty;
pub use gender::GenderProperty;
pub use lang::LanguageProperty;
pub use name::NameProperty;
pub use nickname::NickNameProperty;
pub use photo::PhotoProperty;
pub use rev::RevProperty;
pub use role::RoleProperty;
pub use tel::TelProperty;
pub use title::TitleProperty;
pub use url::URLProperty;
