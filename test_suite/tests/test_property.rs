use vcard::common::VCardProperty;
use vcard::rfc6350::{
    parameters::{BaseType, TelType, VCardType},
    properties::{CategoryProperty, EmailProperty, LanguageProperty, RoleProperty, TitleProperty},
    values::{Category, Email, Language, Role, Title},
};

#[test]
pub fn email_prop() {
    let mut emails = EmailProperty::new();
    emails.add(Email::new().set_value("vyngt@outlook.com"));

    assert_eq!(emails.to_content(), "EMAIL:vyngt@outlook.com\n")
}

#[test]
pub fn email_multiple() {
    let mut emails = EmailProperty::new();
    emails.add(
        Email::new()
            .set_value("vyngt@outlook.com")
            .add_type(VCardType::Base(BaseType::WORK)),
    );

    emails.add(
        Email::new()
            .set_value("ntvy2k@gmail.com")
            .add_type(VCardType::Base(BaseType::HOME))
            .add_type(VCardType::XName("always".into()))
            .add_type(VCardType::Tel(TelType::CELL)) // Invalid, ignore
            .set_prefer(1),
    );

    emails.add(
        Email::new()
            .set_value("") // Empty, then ignore
            .add_type(VCardType::Base(BaseType::WORK)),
    );

    let expected = "\
    EMAIL;TYPE=WORK:vyngt@outlook.com\n\
    EMAIL;PREF=1;TYPE=HOME,ALWAYS:ntvy2k@gmail.com\n";

    assert_eq!(emails.to_content(), expected)
}

#[test]
pub fn lang_prop() {
    let mut languages = LanguageProperty::new();
    languages.add(
        Language::new()
            .set_value("vi")
            .add_type(VCardType::Base(BaseType::WORK))
            .add_type(VCardType::Base(BaseType::HOME))
            .set_prefer(1),
    );

    languages.add(
        Language::new()
            .set_value("en")
            .add_type(VCardType::Base(BaseType::WORK))
            .set_prefer(2),
    );

    languages.add(
        Language::new(), // Ignore
    );

    let expected = "\
    LANG;PREF=1;TYPE=WORK,HOME:vi\n\
    LANG;PREF=2;TYPE=WORK:en\n";

    assert_eq!(languages.to_content(), expected)
}

#[test]
pub fn title_prop() {
    let mut titles = TitleProperty::new();
    titles.add(
        Title::new()
            .set_value("Rustacean")
            .add_type(VCardType::Base(BaseType::WORK))
            .add_type(VCardType::XName("crab".into()))
            .set_prefer(1)
            .set_language(Some("en".into())),
    );

    titles.add(
        Title::new()
            .set_value("Pythonic")
            .add_type(VCardType::Base(BaseType::WORK))
            .set_prefer(2),
    );

    titles.add(
        Title::new(), // Ignore
    );

    let expected = "\
    TITLE;LANGUAGE=en;PREF=1;TYPE=WORK,CRAB:Rustacean\n\
    TITLE;PREF=2;TYPE=WORK:Pythonic\n";

    assert_eq!(titles.to_content(), expected)
}

#[test]
pub fn role_prop() {
    let mut roles = RoleProperty::new();
    roles.add(
        Role::new()
            .set_value("Backend Developer")
            .add_type(VCardType::Base(BaseType::WORK))
            .add_type(VCardType::XName("dark".into()))
            .set_language(Some("en".into()))
            .set_prefer(1),
    );

    roles.add(
        Role::new()
            .set_value("Odoo Developer")
            .add_type(VCardType::Base(BaseType::WORK))
            .set_prefer(10),
    );

    roles.add(
        Role::new(), // Ignore
    );

    let expected = "\
    ROLE;LANGUAGE=en;PREF=1;TYPE=WORK,DARK:Backend Developer\n\
    ROLE;PREF=10;TYPE=WORK:Odoo Developer\n";

    assert_eq!(roles.to_content(), expected)
}

#[test]
pub fn categories_prop() {
    let mut categories = CategoryProperty::new();
    categories.add(
        Category::new()
            .add_category("Rust")
            .add_category("Python")
            .add_category("Javascript")
            .add_type(VCardType::XName("Favorite".into()))
            .set_prefer(1),
    );

    categories.add(
        Category::new()
            .add_category("Odoo")
            .add_type(VCardType::Base(BaseType::WORK))
            .add_type(VCardType::XName("Experienced".into()))
            .set_prefer(50),
    );

    categories.add(
        Category::new(), // Ignore
    );

    let expected = "\
    CATEGORIES;PREF=1;TYPE=FAVORITE:Rust,Python,Javascript\n\
    CATEGORIES;PREF=50;TYPE=WORK,EXPERIENCED:Odoo\n";

    assert_eq!(categories.to_content(), expected)
}
