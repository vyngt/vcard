use regex::Regex;
use std::fs;
use vcard::rfc6350::parameters::{BaseType, VCardType};
use vcard::rfc6350::values::{FullName, IGender, Name, NickName, URL};
use vcard::rfc6350::VCard40;

#[test]
fn vcard_version_4() {
    assert_eq!(VCard40::version(), "4.0");
}

#[test]
#[should_panic(expected = "FullName(FN) Required!")]
fn vcard_required_fn() {
    let vc = VCard40::new();
    vc.generate_vcard();
}

#[test]
fn vcard_simple() {
    let mut vc = VCard40::new();
    vc.name.set(Name::new().add_given_name("Vy"));
    vc.nicknames.add(NickName::new()); // Empty
    vc.full_names.add(
        FullName::new()
            .set_value("Nguyen The Vy")
            .set_language(Some("vi".into())),
    );
    vc.full_names.add(FullName::new());

    let result = vc.to_string();

    assert_eq!(
        result,
        "BEGIN:VCARD\n\
        VERSION:4.0\n\
        FN;LANGUAGE=vi:Nguyen The Vy\n\
        N:;Vy;;;\n\
        END:VCARD"
    );
}

#[test]
fn vcard_write_to_file() {
    let pathname = "../target/tmp/simple_vcard.vcf";
    let pathname_not_exists = "../target/not_exists/simple_vcard.vcf";
    let mut vc = VCard40::new();
    vc.full_names
        .add(FullName::new().set_value("Nguyen The Vy"));

    let expected = "BEGIN:VCARD\n\
    VERSION:4.0\n\
    FN:Nguyen The Vy\n\
    END:VCARD";

    assert_eq!(vc.write_to_file(pathname), true);
    assert_eq!(vc.write_to_file(pathname_not_exists), false);

    let vcf_content = fs::read_to_string(pathname).unwrap();

    assert_eq!(vcf_content, expected);
}

#[test]
fn vcard_with_gender() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");

    vc.full_names.add(fname);
    vc.gender.set(IGender::Male);

    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\nVERSION:4.0\nFN:Nguyen The Vy\nGENDER:M\nEND:VCARD"
    );
}

#[test]
fn vcard_with_urls() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");

    vc.full_names.add(fname);
    vc.urls
        .add(URL::new().set_value("https://github.com/vyngt"));
    vc.urls.add(
        URL::new()
            .set_value("https://leetcode.com/vyngt")
            .add_type(VCardType::XName("Leetcode".into())),
    );
    vc.urls.add(URL::new()); //Empty

    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\n\
        VERSION:4.0\n\
        FN:Nguyen The Vy\n\
        URL:https://github.com/vyngt\n\
        URL;TYPE=LEETCODE:https://leetcode.com/vyngt\n\
        END:VCARD"
    );
}

#[test]
fn vcard_with_birthday() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");

    vc.full_names.add(fname);
    vc.birthday.set(2000, 4, 3);

    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\nVERSION:4.0\nFN:Nguyen The Vy\nBDAY:20000403\nEND:VCARD"
    );
}

#[test]
#[should_panic(expected = "Invalid Date!")]
fn vcard_with_birthday_error() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");

    vc.full_names.add(fname);
    vc.birthday.set(2000, 0, 3);

    vc.generate_vcard();
}

#[test]
#[should_panic(expected = "Invalid Date!")]
fn vcard_with_birthday_error_2() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");

    vc.full_names.add(fname);
    vc.birthday.set(2000, 13, 35);

    vc.generate_vcard();
}

#[test]
fn vcard_with_anniversary() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");

    vc.full_names.add(fname);
    vc.anniversary.set(2023, 12, 12);

    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\nVERSION:4.0\nFN:Nguyen The Vy\nANNIVERSARY:20231212\nEND:VCARD"
    );
}

#[test]
#[should_panic(expected = "Invalid Date!")]
fn vcard_with_anniversary_error() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");

    vc.full_names.add(fname);
    vc.anniversary.set(2000, 0, 3);

    vc.generate_vcard();
}

#[test]
#[should_panic(expected = "Invalid Date!")]
fn vcard_with_anniversary_error_2() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");

    vc.full_names.add(fname);
    vc.anniversary.set(2000, 13, 35);

    vc.generate_vcard();
}

#[test]
fn vcard_with_nicknames() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");
    vc.full_names.add(fname);
    vc.nicknames.add(
        NickName::new()
            .add_nickname("TheVy")
            .add_nickname("Developer")
            .add_nickname(""),
    );
    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\n\
        VERSION:4.0\n\
        FN:Nguyen The Vy\n\
        NICKNAME:TheVy,Developer\n\
        END:VCARD"
    );
}

#[test]
fn vcard_with_nicknames_language() {
    let mut vc = VCard40::new();
    let fname = FullName::new().set_value("Nguyen The Vy");
    vc.full_names.add(fname);
    vc.nicknames.add(
        NickName::new()
            .add_nickname("TheVy")
            .add_nickname("Developer")
            .add_nickname("")
            .set_language(Some("vi".into())),
    );
    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\n\
        VERSION:4.0\n\
        FN:Nguyen The Vy\n\
        NICKNAME;LANGUAGE=vi:TheVy,Developer\n\
        END:VCARD"
    );
}

#[test]
fn vcard_with_types() {
    let mut vc = VCard40::new();

    vc.full_names.add(
        FullName::new()
            .set_value("Nguyen The Vy")
            .add_type(VCardType::Base(BaseType::HOME))
            .add_type(VCardType::XName("dark".into())),
    );
    vc.nicknames.add(
        NickName::new()
            .add_nickname("TheVy")
            .add_nickname("Developer")
            .add_nickname("")
            .add_type(VCardType::Base(BaseType::HOME))
            .add_type(VCardType::Base(BaseType::WORK)),
    );
    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\n\
        VERSION:4.0\n\
        FN;TYPE=HOME,DARK:Nguyen The Vy\n\
        NICKNAME;TYPE=HOME,WORK:TheVy,Developer\n\
        END:VCARD"
    );
}

#[test]
fn vcard_with_rev_current() {
    let mut vc = VCard40::new();
    vc.full_names
        .add(FullName::new().set_value("Nguyen The Vy"));
    vc.rev.update_current();
    let result = vc.generate_vcard();

    let re = Regex::new(r"BEGIN:VCARD\nVERSION:4\.0\nFN:Nguyen The Vy\nREV:\d{8}T\d{6}\nEND:VCARD")
        .unwrap();

    assert!(re.is_match(&result))
}
