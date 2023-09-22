use vcard::rfc6350::VCard40;

#[test]
fn vcard_version_4() {
    assert_eq!(VCard40::version(), "4.0");
}

#[test]
#[should_panic(expected = "Name required!")]
fn required_name() {
    let vc = VCard40::new();
    vc.generate_vcard();
}

#[test]
fn simple_vcard() {
    let mut vc = VCard40::new();
    vc.name.set("Vy", "", "", "", "");
    let result = vc.generate_vcard();

    assert_eq!(result, "BEGIN:VCARD\nVERSION:4.0\nN:;Vy;;;\nEND:VCARD");
}
