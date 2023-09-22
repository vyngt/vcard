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
    vc.full_names.add("Nguyen The Vy");
    let result = vc.generate_vcard();

    assert_eq!(
        result,
        "BEGIN:VCARD\nVERSION:4.0\nFN:Nguyen The Vy\nN:;Vy;;;\nEND:VCARD"
    );
}
