use vcard::{
    common::VCardParam,
    rfc6350::parameters::{BaseType, TelType, TypeParam},
};

#[test]
fn vcard_type_params() {
    let mut type_param = TypeParam::new();

    type_param.push_base(BaseType::HOME);
    type_param.push_tel(TelType::CELL);
    type_param.push_x_name("internet");
    type_param.push_x_name("cus tom");

    let result = type_param.format_param();
    assert_eq!(result, ";TYPE=\"HOME,CELL,INTERNET,CUSTOM\"");
}

#[test]
fn vcard_type_params_empty() {
    let type_param = TypeParam::new();
    let result = type_param.format_param();
    assert_eq!(result, "");
}
