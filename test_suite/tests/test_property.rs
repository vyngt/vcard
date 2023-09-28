use vcard::common::VCardProperty;
use vcard::rfc6350::{
    parameters::{BaseType, TelType, VCardType},
    properties::{EmailProperty, LanguageProperty},
    values::{Email, Language},
};

#[test]
pub fn email_prop() {
    let mut emails = EmailProperty::new();
    emails.push(Email::new().set_value("vyngt@outlook.com"));

    assert_eq!(emails.to_content(), "EMAIL:vyngt@outlook.com\n")
}

#[test]
pub fn email_multiple() {
    let mut emails = EmailProperty::new();
    emails.push(
        Email::new()
            .set_value("vyngt@outlook.com")
            .add_type(VCardType::Base(BaseType::WORK)),
    );

    emails.push(
        Email::new()
            .set_value("ntvy2k@gmail.com")
            .add_type(VCardType::Base(BaseType::HOME))
            .add_type(VCardType::XName("always".into()))
            .add_type(VCardType::Tel(TelType::CELL)) // Invalid, ignore
            .set_prefer(1),
    );

    emails.push(
        Email::new()
            .set_value("") // Empty, then ignore
            .add_type(VCardType::Base(BaseType::WORK)),
    );

    let expected = "\
    EMAIL;TYPE=WORK:vyngt@outlook.com\n\
    EMAIL;PREF=1;TYPE=HOME,ALWAYS:ntvy2k@gmail.com\n";

    assert_eq!(emails.to_content(), expected)
}

#[test]
pub fn lang_prop() {
    let mut languages = LanguageProperty::new();
    languages.push(
        Language::new()
            .set_value("vi")
            .add_type(VCardType::Base(BaseType::WORK))
            .add_type(VCardType::Base(BaseType::HOME))
            .set_prefer(1),
    );

    languages.push(
        Language::new()
            .set_value("en")
            .add_type(VCardType::Base(BaseType::WORK))
            .set_prefer(2),
    );

    languages.push(
        Language::new(), // Ignore
    );

    let expected = "\
    LANG;PREF=1;TYPE=WORK,HOME:vi\n\
    LANG;PREF=2;TYPE=WORK:en\n";

    assert_eq!(languages.to_content(), expected)
}
