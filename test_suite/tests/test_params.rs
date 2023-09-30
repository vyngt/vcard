use sp_vcard::{
    common::VCardParam,
    rfc6350::parameters::{
        media::{AudioType, DataType, ImageType},
        B64MediaValue, BaseType, LanguageParam, PrefParam, TelType, TypeParam, VCardType,
        ValueOrAudioParam, ValueOrImageParam,
    },
};

/// Just for test
///
const SIMPLE_BYTES_DATA: [u8; 130] = [
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 7, 0, 0, 0, 8, 8, 6, 0,
    0, 0, 53, 4, 229, 6, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0, 0, 4, 103, 65,
    77, 65, 0, 0, 177, 143, 11, 252, 97, 5, 0, 0, 0, 9, 112, 72, 89, 115, 0, 0, 18, 116, 0, 0, 18,
    116, 1, 222, 102, 31, 120, 0, 0, 0, 23, 73, 68, 65, 84, 24, 87, 99, 252, 15, 4, 12, 56, 0, 19,
    148, 198, 10, 134, 146, 36, 3, 3, 0, 154, 191, 4, 12, 196, 31, 139, 39, 0, 0, 0, 0, 73, 69, 78,
    68, 174, 66, 96, 130,
];

const SIMPLE_B64_DATA: &str = "iVBORw0KGgoAAAANSUhEUgAAAAcAAAAICAYAAAA1BOUGAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAEnQAABJ0Ad5mH3gAAAAXSURBVBhXY/wPBAw4ABOUxgqGkiQDAwCavwQMxB+LJwAAAABJRU5ErkJggg";

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

#[test]
fn pref_param() {
    let mut pref = PrefParam::new();
    assert_eq!(pref.format_param(), "");

    pref.set(59).unwrap();
    assert_eq!(pref.format_param(), ";PREF=59");
}

#[test]
#[should_panic(expected = "Prefer value condition(x): 1 <= x <= 100")]
fn pref_param_invalid() {
    let mut pref = PrefParam::new();
    pref.set(101).unwrap();
}

#[test]
fn b64_media_empty() {
    let b64_data = B64MediaValue::new();
    assert_eq!(b64_data.to_string(), "");
}

#[test]
fn b64_media_img_png() {
    let mut b64_img = B64MediaValue::new();
    b64_img.set(SIMPLE_BYTES_DATA.to_vec(), DataType::Image(ImageType::PNG));
    let expected = format!("data:image/png;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(b64_img.to_string(), expected);
}

#[test]
fn b64_media_img_jpeg() {
    let mut b64_img = B64MediaValue::new();
    b64_img.set(SIMPLE_BYTES_DATA.to_vec(), DataType::Image(ImageType::JPEG));
    let expected = format!("data:image/jpeg;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(b64_img.to_string(), expected);
}

#[test]
fn b64_media_img_gif() {
    let mut b64_img = B64MediaValue::new();
    b64_img.set(SIMPLE_BYTES_DATA.to_vec(), DataType::Image(ImageType::GIF));
    let expected = format!("data:image/gif;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(b64_img.to_string(), expected);
}

#[test]
fn b64_media_img_bmp() {
    let mut b64_img = B64MediaValue::new();
    b64_img.set(SIMPLE_BYTES_DATA.to_vec(), DataType::Image(ImageType::BMP));
    let expected = format!("data:image/bmp;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(b64_img.to_string(), expected);
}

#[test]
fn b64_media_audio_basic() {
    let mut b64_audio = B64MediaValue::new();
    b64_audio.set(
        SIMPLE_BYTES_DATA.to_vec(),
        DataType::Audio(AudioType::BASIC),
    );
    let expected = format!("data:audio/basic;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(b64_audio.to_string(), expected);
}

#[test]
fn b64_media_audio_mpeg() {
    let mut b64_audio = B64MediaValue::new();
    b64_audio.set(SIMPLE_BYTES_DATA.to_vec(), DataType::Audio(AudioType::MPEG));
    let expected = format!("data:audio/mpeg;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(b64_audio.to_string(), expected);
}

#[test]
fn b64_media_audio_mp4() {
    let mut b64_audio = B64MediaValue::new();
    b64_audio.set(SIMPLE_BYTES_DATA.to_vec(), DataType::Audio(AudioType::MP4));
    let expected = format!("data:audio/mp4;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(b64_audio.to_string(), expected);
}

#[test]
fn b64_media_audio_ogg() {
    let mut b64_audio = B64MediaValue::new();
    b64_audio.set(SIMPLE_BYTES_DATA.to_vec(), DataType::Audio(AudioType::OGG));
    let expected = format!("data:audio/ogg;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(b64_audio.to_string(), expected);
}

#[test]
fn value_or_image_param() {
    let mut voi = ValueOrImageParam::new();
    assert_eq!(voi.to_string(), "");

    voi.set_uri("https://image-storage.example.com/p/1");
    assert_eq!(voi.to_string(), "https://image-storage.example.com/p/1");

    voi.set_bytes_data(SIMPLE_BYTES_DATA.to_vec(), ImageType::PNG);
    let expected = format!("data:image/png;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(voi.to_string(), expected);
}

#[test]
fn value_or_audio_param() {
    let mut voi = ValueOrAudioParam::new();
    assert_eq!(voi.to_string(), "");

    voi.set_uri("https://music-clound.example.com/hello-world.mp3");
    assert_eq!(
        voi.to_string(),
        "https://music-clound.example.com/hello-world.mp3"
    );

    voi.set_bytes_data(SIMPLE_BYTES_DATA.to_vec(), AudioType::BASIC);
    let expected = format!("data:audio/basic;base64,{}", SIMPLE_B64_DATA);
    assert_eq!(voi.to_string(), expected);
}
