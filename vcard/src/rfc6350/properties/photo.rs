use super::super::values::Photo;
use crate::common::{VCardProperty, VCardValue};

pub struct PhotoProperty {
    photos: Vec<Photo>,
}

impl PhotoProperty {
    pub fn new() -> Self {
        PhotoProperty { photos: vec![] }
    }

    pub fn add(&mut self, photo: Photo) {
        self.photos.push(photo);
    }
}

impl VCardProperty for PhotoProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for photo in &self.photos {
            output.push_str(&photo.format_value());
        }

        output
    }
}
