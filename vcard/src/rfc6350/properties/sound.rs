use super::super::values::Sound;
use crate::common::{VCardProperty, VCardValue};

pub struct SoundProperty {
    sounds: Vec<Sound>,
}

impl SoundProperty {
    pub fn new() -> Self {
        Self { sounds: vec![] }
    }

    pub fn add(&mut self, sound: Sound) {
        self.sounds.push(sound);
    }
}

impl VCardProperty for SoundProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for sound in &self.sounds {
            output.push_str(&sound.format_value());
        }

        output
    }
}
