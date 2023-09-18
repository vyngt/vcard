use vcard::rfc6530;

#[test]
fn test_add() {
    assert_eq!(rfc6530::add(1, 2), 3);
}
