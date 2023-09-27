use vcard::common::VCardProperty;
use vcard::rfc6350::{
    parameters::{BaseType, TelType, VCardType},
    properties::EmailProperty,
    values::Email,
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
