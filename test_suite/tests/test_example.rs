use sp_vcard::rfc6350::parameters::{BaseType, VCardType};
use sp_vcard::rfc6350::values::{
    Category, Email, FullName, IGender, Language, NickName, Role, Title, URL,
};
use sp_vcard::rfc6350::VCard40;

#[test]
fn example_1() {
    let mut vc = VCard40::new();

    vc.full_names.add(
        FullName::new()
            .set_value("Nguyen The Vy")
            .set_language(Some("vi".into()))
            .add_type(VCardType::Base(BaseType::HOME))
            .add_type(VCardType::Base(BaseType::WORK)),
    );

    vc.gender.set(IGender::Male);
    vc.emails.add(
        Email::new()
            .set_value("vyngt@outlook.com")
            .set_prefer(1)
            .add_type(VCardType::Base(BaseType::WORK)),
    );
    vc.emails.add(
        Email::new()
            .set_value("ntvy2k@gmail.com")
            .set_prefer(2)
            .add_type(VCardType::Base(BaseType::HOME)),
    );

    vc.languages
        .add(Language::new().set_prefer(1).set_value("vi"));
    vc.categories.add(
        Category::new()
            .add_category("Rust")
            .add_category("Python")
            .add_category("Javascript"),
    );
    vc.nicknames
        .add(NickName::new().add_nickname("TheVy").add_nickname("VyNT"));
    vc.urls.add(
        URL::new()
            .set_value("https://github.com/vyngt")
            .add_type(VCardType::XName("Github".into())),
    );
    vc.titles.add(
        Title::new()
            .set_value("Rust Developer")
            .add_type(VCardType::Base(BaseType::HOME)),
    );
    vc.titles.add(
        Title::new()
            .set_value("Python Developer")
            .add_type(VCardType::Base(BaseType::WORK)),
    );
    vc.roles.add(Role::new().set_value("Story Teller"));

    let expected = "BEGIN:VCARD\n\
    VERSION:4.0\n\
    FN;LANGUAGE=vi;TYPE=HOME,WORK:Nguyen The Vy\n\
    NICKNAME:TheVy,VyNT\n\
    GENDER:M\n\
    EMAIL;PREF=1;TYPE=WORK:vyngt@outlook.com\n\
    EMAIL;PREF=2;TYPE=HOME:ntvy2k@gmail.com\n\
    LANG;PREF=1:vi\n\
    TITLE;TYPE=HOME:Rust Developer\n\
    TITLE;TYPE=WORK:Python Developer\n\
    ROLE:Story Teller\n\
    CATEGORIES:Rust,Python,Javascript\n\
    URL;TYPE=GITHUB:https://github.com/vyngt\n\
    END:VCARD";

    assert_eq!(vc.to_string(), expected);
}
