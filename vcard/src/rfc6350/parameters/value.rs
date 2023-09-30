use super::media::{AudioType, B64MediaValue, DataType, ImageType};
use crate::common::VCardParam;
/// Just Text
pub struct ValueParam {
    value: String,
}

impl ValueParam {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn set(&mut self, value: &str) {
        let trimmed = value.trim();
        if trimmed.len() > 0 {
            self.value = trimmed.into();
        }
    }
}

impl VCardParam for ValueParam {
    fn format_param(&self) -> String {
        self.value.clone()
    }
}

impl std::fmt::Display for ValueParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_param())
    }
}

/// Multiple Text: repeated in COMMA-separated.
///
/// Only: N, NICKNAME, ADR, and CATEGORIES
pub struct ArrayValueParam {
    value: Vec<String>,
}

impl ArrayValueParam {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub fn add(&mut self, value: &str) {
        let trimmed = value.trim();
        if trimmed.len() > 0 {
            self.value.push(trimmed.into());
        }
    }
}

impl VCardParam for ArrayValueParam {
    fn format_param(&self) -> String {
        self.value.join(",").to_string()
    }
}

impl std::fmt::Display for ArrayValueParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_param())
    }
}

// ----------------------- IMAGE
pub enum ValueOrImage {
    Value(ValueParam),
    Image(B64MediaValue),
}

impl std::fmt::Display for ValueOrImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self {
            ValueOrImage::Value(data) => data.to_string(),
            ValueOrImage::Image(data) => data.to_string(),
        };

        write!(f, "{}", output)
    }
}

/// Using for PHOTO and LOGO
pub struct ValueOrImageParam {
    data: ValueOrImage,
}

impl ValueOrImageParam {
    pub fn new() -> Self {
        Self {
            data: ValueOrImage::Value(ValueParam { value: "".into() }),
        }
    }

    pub fn set_uri(&mut self, uri: &str) {
        let mut new_data = ValueParam::new();
        new_data.set(uri);
        self.data = ValueOrImage::Value(new_data);
    }

    pub fn set_bytes_data(&mut self, data: Vec<u8>, datatype: ImageType) {
        let mut new_data = B64MediaValue::new();
        new_data.set(data, DataType::Image(datatype));
        self.data = ValueOrImage::Image(new_data);
    }
}

impl std::fmt::Display for ValueOrImageParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.to_string())
    }
}

// ---------------------- AUDIO
pub enum ValueOrAudio {
    Value(ValueParam),
    Audio(B64MediaValue),
}

impl std::fmt::Display for ValueOrAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self {
            ValueOrAudio::Value(data) => data.to_string(),
            ValueOrAudio::Audio(data) => data.to_string(),
        };

        write!(f, "{}", output)
    }
}

/// Using for SOUND
pub struct ValueOrAudioParam {
    data: ValueOrAudio,
}

impl ValueOrAudioParam {
    pub fn new() -> Self {
        Self {
            data: ValueOrAudio::Value(ValueParam { value: "".into() }),
        }
    }

    pub fn set_uri(&mut self, uri: &str) {
        let mut new_data = ValueParam::new();
        new_data.set(uri);
        self.data = ValueOrAudio::Value(new_data);
    }

    pub fn set_bytes_data(&mut self, data: Vec<u8>, datatype: AudioType) {
        let mut new_data = B64MediaValue::new();
        new_data.set(data, DataType::Audio(datatype));
        self.data = ValueOrAudio::Audio(new_data);
    }
}

impl std::fmt::Display for ValueOrAudioParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.to_string())
    }
}
