//! pictures, gif, etc...
use crate::base64::{engine::general_purpose, Engine as _};
use crate::common::VCardParam;
pub enum ImageType {
    JPEG,
    PNG,
    GIF,
    BMP,
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let it = match self {
            ImageType::JPEG => "image/jpeg",
            ImageType::PNG => "image/png",
            ImageType::GIF => "image/gif",
            ImageType::BMP => "image/bmp",
        };

        write!(f, "{}", it.to_string())
    }
}

pub enum AudioType {
    BASIC,
    MPEG,
    MP4,
    OGG,
}

impl std::fmt::Display for AudioType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let it = match self {
            AudioType::BASIC => "audio/basic",
            AudioType::MPEG => "audio/mpeg",
            AudioType::MP4 => "audio/mp4",
            AudioType::OGG => "audio/ogg",
        };

        write!(f, "{}", it)
    }
}

pub enum DataType {
    Image(ImageType),
    Audio(AudioType),
    None,
}

pub struct B64MediaValue {
    data: Vec<u8>,
    datatype: DataType,
}

impl B64MediaValue {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            datatype: DataType::None,
        }
    }

    pub fn set(&mut self, data: Vec<u8>, datatype: DataType) {
        self.data.clear();
        self.data.extend(data);
        self.datatype = datatype;
    }

    pub fn data_b64(&self) -> String {
        general_purpose::STANDARD_NO_PAD.encode(&self.data)
    }
}

impl VCardParam for B64MediaValue {
    fn format_param(&self) -> String {
        let datatype: String = match &self.datatype {
            DataType::Image(dt) => dt.to_string(),
            DataType::Audio(dt) => dt.to_string(),
            DataType::None => "".into(),
        };
        if self.data.len() == 0 || datatype.len() == 0 {
            "".into()
        } else {
            format!("data:{};base64,{}", datatype, self.data_b64())
        }
    }
}

impl std::fmt::Display for B64MediaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_param())
    }
}
