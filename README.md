### Simple vCard library

Simple vCard library will help you generate vCard

### Example

```rust
use std::fs;
use sp_vcard::rfc6350::parameters::BaseType;
use sp_vcard::rfc6350::values::{
    Category, Email, FullName, IGender, Role, Title, Photo,
};
use sp_vcard::rfc6350::parameters::media::ImageType;
use sp_vcard::rfc6350::VCard40;

fn main() {
    let mut vc = VCard40::new();

    vc.full_names.add(
        FullName::new()
            .set_value("Hello World")
            .set_language(Some("en".into()))
            .add_base_type(BaseType::HOME)
            .add_base_type(BaseType::WORK),
    );
    vc.gender.set(IGender::Male);
    vc.emails.add(
        Email::new()
            .set_value("user@example.com")
            .add_base_type(BaseType::WORK),
    );
    vc.categories.add(
        Category::new()
            .add_category("Rust")
    );
    vc.titles.add(
        Title::new()
            .set_value("Rust Developer")
            .add_base_type(BaseType::HOME),
    );
    vc.roles.add(Role::new().set_value("Story Teller"));

    let image = fs::read("./simple_image.png").unwrap();

    vc.photos
        .add(Photo::new().set_bytes_data(image, ImageType::PNG));

    println!("{}", vc);
}


```
