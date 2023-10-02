use sp_vcard::common::VCardProperty;
use sp_vcard::rfc6350::{
    parameters::{
        media::{AudioType, ImageType},
        BaseType, TelType,
    },
    properties::{
        AddressProperty, CategoryProperty, EmailProperty, FullNameProperty, LanguageProperty,
        LogoProperty, NameProperty, NickNameProperty, NoteProperty, OrganizationProperty,
        PhotoProperty, RoleProperty, SoundProperty, SourceProperty, TelProperty, TitleProperty,
        URLProperty,
    },
    values::{
        Address, Category, Email, FullName, Language, Logo, Name, NickName, Note, Organization,
        Photo, Role, Sound, Source, Tel, Title, URL,
    },
};

#[test]
fn name_property() {
    let mut name = NameProperty::new();

    name.set(
        Name::new()
            .add_family_name("Last")
            .add_given_name("First")
            .add_additional_name("Middle")
            .add_honorific_prefix("Pre.")
            .add_honorific_suffix("Suf.")
            .set_language(Some("en")),
    );

    let expected = "N;LANGUAGE=en:Last;First;Middle;Pre.;Suf.\n";

    assert_eq!(name.to_content(), expected);
}

#[test]
fn full_name_property() {
    let mut full_names = FullNameProperty::new();

    full_names.add(
        FullName::new()
            .set_value("Nguyen The Vy")
            .set_prefer(1)
            .set_language(Some("vi"))
            .add_base_type(BaseType::HOME)
            .add_base_type(BaseType::WORK),
    );

    full_names.add(
        FullName::new()
            .set_value("Vy Nguyen The")
            .set_prefer(2)
            .set_language(Some("en"))
            .add_base_type(BaseType::WORK),
    );

    full_names.add(FullName::new()); //Ignore

    let expected = "\
    FN;PREF=1;LANGUAGE=vi;TYPE=HOME,WORK:Nguyen The Vy\n\
    FN;PREF=2;LANGUAGE=en;TYPE=WORK:Vy Nguyen The\n";

    assert_eq!(full_names.to_content(), expected);
}

#[test]
fn nickname_property() {
    let mut nicknames = NickNameProperty::new();

    nicknames.add(
        NickName::new()
            .add_nickname("TheVy")
            .add_nickname("Grr")
            .set_prefer(1)
            .set_language(None)
            .add_x_type("Gaming"),
    );

    nicknames.add(
        NickName::new()
            .add_nickname("VyNT")
            .set_prefer(2)
            .set_language(Some("en"))
            .add_base_type(BaseType::WORK),
    );

    nicknames.add(NickName::new()); //Ignore

    let expected = "\
    NICKNAME;PREF=1;TYPE=GAMING:TheVy,Grr\n\
    NICKNAME;PREF=2;LANGUAGE=en;TYPE=WORK:VyNT\n";

    assert_eq!(nicknames.to_content(), expected);
}

#[test]
fn url_property() {
    let mut urls = URLProperty::new();

    urls.add(
        URL::new()
            .set_value("https://github.com/vyngt")
            .set_prefer(1)
            .add_x_type("Github"),
    );

    urls.add(
        URL::new()
            .set_value("https://example.com")
            .set_prefer(2)
            .add_base_type(BaseType::WORK),
    );

    urls.add(URL::new()); //Ignore

    let expected = "\
    URL;PREF=1;TYPE=GITHUB:https://github.com/vyngt\n\
    URL;PREF=2;TYPE=WORK:https://example.com\n";

    assert_eq!(urls.to_content(), expected);
}

#[test]
fn email_property() {
    let mut emails = EmailProperty::new();
    emails.add(Email::new().set_value("vyngt@outlook.com"));

    assert_eq!(emails.to_content(), "EMAIL:vyngt@outlook.com\n")
}

#[test]
fn email_multiple() {
    let mut emails = EmailProperty::new();
    emails.add(
        Email::new()
            .set_value("vyngt@outlook.com")
            .add_base_type(BaseType::WORK),
    );

    emails.add(
        Email::new()
            .set_value("ntvy2k@gmail.com")
            .add_base_type(BaseType::HOME)
            .add_x_type("always")
            .set_prefer(1),
    );

    emails.add(
        Email::new()
            .set_value("") // Empty, then ignore
            .add_base_type(BaseType::WORK),
    );

    let expected = "\
    EMAIL;TYPE=WORK:vyngt@outlook.com\n\
    EMAIL;PREF=1;TYPE=HOME,ALWAYS:ntvy2k@gmail.com\n";

    assert_eq!(emails.to_content(), expected)
}

#[test]
fn lang_property() {
    let mut languages = LanguageProperty::new();
    languages.add(
        Language::new()
            .set_value("vi")
            .add_base_type(BaseType::WORK)
            .add_base_type(BaseType::HOME)
            .set_prefer(1),
    );

    languages.add(
        Language::new()
            .set_value("en")
            .add_base_type(BaseType::WORK)
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
fn title_property() {
    let mut titles = TitleProperty::new();
    titles.add(
        Title::new()
            .set_value("Rustacean")
            .add_base_type(BaseType::WORK)
            .add_x_type("crab")
            .set_prefer(1)
            .set_language(Some("en")),
    );

    titles.add(
        Title::new()
            .set_value("Pythonic")
            .add_base_type(BaseType::WORK)
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
fn role_property() {
    let mut roles = RoleProperty::new();
    roles.add(
        Role::new()
            .set_value("Backend Developer")
            .add_base_type(BaseType::WORK)
            .add_x_type("dark")
            .set_language(Some("en"))
            .set_prefer(1),
    );

    roles.add(
        Role::new()
            .set_value("Odoo Developer")
            .add_base_type(BaseType::WORK)
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
fn categories_property() {
    let mut categories = CategoryProperty::new();
    categories.add(
        Category::new()
            .add_category("Rust")
            .add_category("Python")
            .add_category("Javascript")
            .add_x_type("Favorite")
            .set_prefer(1),
    );

    categories.add(
        Category::new()
            .add_category("Odoo")
            .add_base_type(BaseType::WORK)
            .add_x_type("Experienced")
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

#[test]
fn tel_property() {
    let mut tels = TelProperty::new();
    tels.add(
        Tel::new()
            .set_value("+841216214830")
            .set_prefer(1)
            .add_tel_type(TelType::CELL)
            .add_base_type(BaseType::WORK),
    );

    tels.add(
        Tel::new()
            .set_value("+841218189118")
            .set_prefer(2)
            .add_tel_type(TelType::VOICE)
            .add_base_type(BaseType::HOME),
    );

    tels.add(
        Tel::new(), // Ignore
    );

    let expected = "\
    TEL;PREF=1;TYPE=CELL,WORK:+841216214830\n\
    TEL;PREF=2;TYPE=VOICE,HOME:+841218189118\n";

    assert_eq!(tels.to_content(), expected)
}

#[test]
fn organization_property() {
    let mut orgs = OrganizationProperty::new();
    orgs.add(Organization::new().set_value("My Company"));

    orgs.add(
        Organization::new()
            .set_value("Dream Company")
            .add_ou("H Division")
            .add_ou("Researcher")
            .set_language(Some("en"))
            .set_prefer(1)
            .add_base_type(BaseType::WORK)
            .add_x_type("dream"),
    );

    orgs.add(
        Organization::new(), // Ignore
    );

    let expected = "\
    ORG:My Company\n\
    ORG;PREF=1;LANGUAGE=en;TYPE=WORK,DREAM:Dream Company;H Division;Researcher\n";

    assert_eq!(orgs.to_content(), expected)
}

#[test]
fn note_property() {
    let mut notes = NoteProperty::new();

    notes.add(Note::new().set_value("This is just note"));
    notes.add(Note::new()); // Ignore
    notes.add(
        Note::new()
            .set_value("I love anime, light novel!")
            .set_language(Some("en"))
            .set_prefer(1)
            .add_base_type(BaseType::HOME),
    );

    let expected = "\
    NOTE:This is just note\n\
    NOTE;PREF=1;LANGUAGE=en;TYPE=HOME:I love anime, light novel!\n";
    assert_eq!(notes.to_content(), expected)
}

#[test]
fn address_property() {
    let mut addresses = AddressProperty::new();

    addresses.add(
        Address::new()
            .street("123 Main Street")
            .locality("Town")
            .region("Unknown")
            .code("12344321")
            .country("Country")
            .set_prefer(1)
            .add_base_type(BaseType::HOME),
    );

    addresses.add(Address::new().country("Vietnam"));
    addresses.add(Address::new()); // Ignore

    let expected = "\
    ADR;PREF=1;TYPE=HOME:;;123 Main Street;Town;Unknown;12344321;Country\n\
    ADR:;;;;;;Vietnam\n";
    assert_eq!(addresses.to_content(), expected)
}

#[test]
fn photo_property() {
    let mut photos = PhotoProperty::new();

    photos.add(Photo::new().set_uri("https://example.com/me.png"));
    photos.add(Photo::new().set_bytes_data(
        vec![
            137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 7, 0, 0, 0, 8,
            8, 6, 0, 0, 0, 53, 4, 229, 6, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0,
            0, 4, 103, 65, 77, 65, 0, 0, 177, 143, 11, 252, 97, 5, 0, 0, 0, 9, 112, 72, 89, 115, 0,
            0, 18, 116, 0, 0, 18, 116, 1, 222, 102, 31, 120, 0, 0, 0, 23, 73, 68, 65, 84, 24, 87,
            99, 252, 15, 4, 12, 56, 0, 19, 148, 198, 10, 134, 146, 36, 3, 3, 0, 154, 191, 4, 12,
            196, 31, 139, 39, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
        ],
        ImageType::PNG,
    ));
    photos.add(Photo::new()); // Ignore

    let expected = "\
    PHOTO:https://example.com/me.png\n\
    PHOTO:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAcAAAAICAYAAAA1BOUGAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAEnQAABJ0Ad5mH3gAAAAXSURBVBhXY/wPBAw4ABOUxgqGkiQDAwCavwQMxB+LJwAAAABJRU5ErkJggg\n";

    assert_eq!(photos.to_content(), expected);
}

#[test]
fn logo_property() {
    let mut logos = LogoProperty::new();

    logos.add(Logo::new().set_uri("https://example.com/me.png"));
    logos.add(Logo::new().set_bytes_data(
        vec![
            137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 7, 0, 0, 0, 8,
            8, 6, 0, 0, 0, 53, 4, 229, 6, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0,
            0, 4, 103, 65, 77, 65, 0, 0, 177, 143, 11, 252, 97, 5, 0, 0, 0, 9, 112, 72, 89, 115, 0,
            0, 18, 116, 0, 0, 18, 116, 1, 222, 102, 31, 120, 0, 0, 0, 23, 73, 68, 65, 84, 24, 87,
            99, 252, 15, 4, 12, 56, 0, 19, 148, 198, 10, 134, 146, 36, 3, 3, 0, 154, 191, 4, 12,
            196, 31, 139, 39, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
        ],
        ImageType::PNG,
    ));
    logos.add(Logo::new()); // Ignore

    let expected = "\
    LOGO:https://example.com/me.png\n\
    LOGO:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAcAAAAICAYAAAA1BOUGAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAEnQAABJ0Ad5mH3gAAAAXSURBVBhXY/wPBAw4ABOUxgqGkiQDAwCavwQMxB+LJwAAAABJRU5ErkJggg\n";

    assert_eq!(logos.to_content(), expected);
}

#[test]
fn sound_property() {
    let mut sounds = SoundProperty::new();

    // blah blah blah...
    let sound_stream = vec![
        255, 243, 228, 100, 0, 32, 209, 110, 182, 0, 173, 224, 0, 151, 208, 105, 216, 23, 76, 8, 0,
        62, 45, 35, 180, 134, 3, 76, 25, 155, 153, 209, 226, 158, 239, 73, 245, 239, 158, 77, 73,
        191, 164, 153, 160, 113, 134, 15, 25, 162, 97, 179, 63, 28, 165, 65, 220, 95, 29, 197, 33,
        189, 24, 24, 250, 249, 225, 106, 30, 198, 49, 214, 70, 27,
    ];

    sounds.add(Sound::new().set_uri("https://example.com/my_sound.mp3"));
    sounds.add(Sound::new().set_bytes_data(sound_stream, AudioType::BASIC));
    sounds.add(Sound::new()); // Ignore

    let expected = "\
    SOUND:https://example.com/my_sound.mp3\n\
    SOUND:data:audio/basic;base64,//PkZAAg0W62AK3gAJfQadgXTAgAPi0jtIYDTBmbmdHinu9J9e+eTUm/pJmgcYYPGaJhsz8cpUHcXx3FIb0YGPr54WoexjHWRhs\n";
    assert_eq!(sounds.to_content(), expected);
}

#[test]
fn source_property() {
    let mut sources = SourceProperty::new();

    sources.add(
        Source::new()
            .set_prefer(1)
            .set_value("https://example.com/contacts/vynt.vcf"),
    );
    sources.add(
        Source::new().set_value("ldap://ldap.example.com/cn=Babs%20Jensen,%20o=Babsco,%20c=US"),
    );

    sources.add(Source::new()); // Ignore

    let expected = "\
    SOURCE;PREF=1:https://example.com/contacts/vynt.vcf\n\
    SOURCE:ldap://ldap.example.com/cn=Babs%20Jensen,%20o=Babsco,%20c=US\n";

    assert_eq!(sources.to_content(), expected);
}
