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
pub mod gender;
pub mod name;
pub mod photo;

pub use gender::Gender;
pub use name::{FullName, Name, NickName};
