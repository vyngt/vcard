use vcard::{
    common::VCardParam,
    rfc6350::parameters::{BaseType, LanguageParam, TelType, TypeParam, VCardType},
};

#[test]
fn vcard_type_params() {
    let type_param = TypeParam::new()
        .add_base(BaseType::HOME)
        .add_tel(TelType::CELL)
        .add_x_name("internet")
        .add_x_name("cus tom")
        .add_x_name("")
        .add_x_name("   ")
        .add(VCardType::XName("  ".into())); //Low Level

    let result = type_param.format_param();
    assert_eq!(result, ";TYPE=HOME,CELL,INTERNET,CUSTOM");
}

#[test]
fn vcard_type_params_empty() {
    let type_param = TypeParam::new();
    let result = type_param.format_param();
    assert_eq!(result, "");
}

#[test]
fn language_param_empty() {
    let lang = LanguageParam::new();

    assert_eq!(lang.format_param(), "");
}

#[test]
fn language_param() {
    let mut lang = LanguageParam::new();
    lang.set(Some("vi".into()));

    assert_eq!(lang.format_param(), ";LANGUAGE=vi");
}
