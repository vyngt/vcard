use sp_vcard::common::VCardValue;
use sp_vcard::rfc6350::values::{Gender, IGender};

#[test]
fn output_gender() {
    let mut gender = Gender::new();

    gender.set(IGender::None);
    assert_eq!(gender.format_value(), "");

    gender.set(IGender::Male);
    assert_eq!(gender.format_value(), "GENDER:M\n");

    gender.set(IGender::Female);
    assert_eq!(gender.format_value(), "GENDER:F\n");

    gender.set(IGender::Other);
    assert_eq!(gender.format_value(), "GENDER:O\n");

    gender.set(IGender::NotApplicable);
    assert_eq!(gender.format_value(), "GENDER:N\n");

    gender.set(IGender::Unknown);
    assert_eq!(gender.format_value(), "GENDER:U\n");
}
