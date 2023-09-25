use vcard::rfc6350::parameters::{BaseVCType, TelVCType, VCardTypeParams};

#[test]
fn vcard_type_params() {
    let mut types = VCardTypeParams::new();

    types.push_base(BaseVCType::HOME);
    types.push_tel(TelVCType::CELL);
    types.push_x_name("internet");
    types.push_x_name("cus tom");

    let result = types.to_value();
    assert_eq!(result, "HOME,CELL,INTERNET,CUSTOM");
}

#[test]
fn vcard_type_params_empty() {
    let types = VCardTypeParams::new();
    let result = types.to_value();
    assert_eq!(result, "");
}
