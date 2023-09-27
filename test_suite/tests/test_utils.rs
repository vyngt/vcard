use vcard::utils::DateTimeFormatter;

#[test]
fn date_time_valid() {
    let dt = DateTimeFormatter::fmt_date_time(2023, 9, 27, 23, 33, 30).unwrap();

    assert_eq!(dt, "20230927T233330");
}

#[test]
#[should_panic]
fn date_time_invalid() {
    DateTimeFormatter::fmt_date_time(2023, 13, 27, 23, 33, 30).unwrap();
}

#[test]
fn time_valid() {
    let dt = DateTimeFormatter::fmt_time(23, 33, 30).unwrap();

    assert_eq!(dt, "233330");
}

#[test]
#[should_panic]
fn time_invalid() {
    DateTimeFormatter::fmt_time(2023, 13, 27).unwrap();
}

#[test]
fn date_valid() {
    let dt = DateTimeFormatter::fmt_date(2023, 9, 27).unwrap();

    assert_eq!(dt, "20230927");
}

#[test]
#[should_panic]
fn date_invalid() {
    DateTimeFormatter::fmt_date(2023, 13, 32).unwrap();
}
