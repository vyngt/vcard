use super::super::values::Photo;
use crate::common::{VCardProperty, VCardValue};

/// # Photo
/// ref: `https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.4`
///
/// # Example
/// ```
/// use sp_vcard::rfc6350::values::{Photo, FullName};
/// use sp_vcard::rfc6350::VCard40;
/// use sp_vcard::rfc6350::parameters::media::ImageType;
///
/// let mut vc = VCard40::new();
///
/// vc.full_names.add(
///     FullName::new().set_value("Full Name")
/// );
///
/// vc.photos.add(Photo::new().set_uri("https://example.com/pictures/1"));
/// // Or
/// let example_image_bytes_data = vec![
///    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 7, 0, 0, 0, 8,
///     8, 6, 0, 0, 0, 53, 4, 229, 6, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0,
///     0, 4, 103, 65, 77, 65, 0, 0, 177, 143, 11, 252, 97, 5, 0, 0, 0, 9, 112, 72, 89, 115, 0,
///     0, 18, 116, 0, 0, 18, 116, 1, 222, 102, 31, 120, 0, 0, 0, 23, 73, 68, 65, 84, 24, 87,
///     99, 252, 15, 4, 12, 56, 0, 19, 148, 198, 10, 134, 146, 36, 3, 3, 0, 154, 191, 4, 12,
///     196, 31, 139, 39, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
/// ];
/// // Or something like `std::fs::read("./image.png")`
///
/// vc.photos.add(Photo::new().set_bytes_data(example_image_bytes_data, ImageType::PNG));
///
/// let expected = "BEGIN:VCARD\n\
/// VERSION:4.0\n\
/// FN:Full Name\n\
/// PHOTO:https://example.com/pictures/1\n\
/// PHOTO:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAcAAAAICAYAAAA1BOUGAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAEnQAABJ0Ad5mH3gAAAAXSURBVBhXY/wPBAw4ABOUxgqGkiQDAwCavwQMxB+LJwAAAABJRU5ErkJggg\n\
/// END:VCARD";
///
/// assert_eq!(vc.to_string(), expected);
/// ```
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
