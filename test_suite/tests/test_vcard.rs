use vcard::rfc6350::parameters::{BaseVCType, VCardType};
use vcard::rfc6350::values::{FullName, IGender, NickName};
use vcard::rfc6350::VCard40;

#[test]
fn vcard_version_4() {
    assert_eq!(VCard40::version(), "4.0");
}

#[test]
#[should_panic(expected = "FullName(FN) Required!")]
fn required_name() {
    let vc = VCard40::new();
    vc.generate_vcard();
}

#[test]
fn simple_vcard() {
    let mut vc = VCard40::new();
    vc.name.set("Vy", "", "", "", "");
    vc.nicknames.push(NickName::new()); // Empty

    let fname = FullName::new("Nguyen The Vy");
    vc.full_names.push(fname);
    let fname2 = FullName::new(""); // Empty
    vc.full_names.push(fname2);
    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\nVERSION:4.0\nFN:Nguyen The Vy\nN:;Vy;;;\nEND:VCARD"
    );
}

#[test]
fn vcard_with_gender() {
    let mut vc = VCard40::new();
    let fname = FullName::new("Nguyen The Vy");
    vc.full_names.push(fname);
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
    let fname = FullName::new("Nguyen The Vy");
    vc.full_names.push(fname);
    vc.urls.push_url("https://github.com/vyngt");
    vc.urls.push_url("https://leetcode.com/vyngt");

    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\nVERSION:4.0\nFN:Nguyen The Vy\nURL:https://github.com/vyngt\nURL:https://leetcode.com/vyngt\nEND:VCARD"
    );
}

#[test]
fn vcard_with_birthday() {
    let mut vc = VCard40::new();
    let fname = FullName::new("Nguyen The Vy");
    vc.full_names.push(fname);
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
    let fname = FullName::new("Nguyen The Vy");
    vc.full_names.push(fname);
    vc.birthday.set(2000, 0, 3);

    vc.generate_vcard();
}

#[test]
#[should_panic(expected = "Invalid Date!")]
fn vcard_with_birthday_error_2() {
    let mut vc = VCard40::new();
    let fname = FullName::new("Nguyen The Vy");
    vc.full_names.push(fname);
    vc.birthday.set(2000, 13, 35);

    vc.generate_vcard();
}

#[test]
fn vcard_with_nicknames() {
    let mut vc = VCard40::new();
    let fname = FullName::new("Nguyen The Vy");
    vc.full_names.push(fname);
    vc.nicknames.push(
        NickName::new()
            .add_nickname("TheVy")
            .add_nickname("Developer")
            .add_nickname("")
            .add_type(VCardType::Base(BaseVCType::HOME))
            .add_type(VCardType::Base(BaseVCType::WORK)),
    );
    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\nVERSION:4.0\nFN:Nguyen The Vy\nNICKNAME;TYPE=\"HOME,WORK\":TheVy,Developer\nEND:VCARD"
    );
}
