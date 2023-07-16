use std::str::FromStr;
use a1_notation::A1;

#[test]
fn test_a1_to_and_from() {
    assert_eq!(
        "A1",
        A1::from_str("A1").unwrap().to_string());

    assert_eq!(
        "Foo!A:C",
        A1::from_str("Foo!A:C").unwrap().to_string());
}

#[test]
fn test_a1_builder() {
    assert!(A1::builder()
            .x(5)
            .y(10)
            .sheet_name("foo")
            .build()
            .is_ok());

    assert!(A1::builder()
            .range()
            .from(A1::builder().x(5).build().unwrap())
            .to(A1::builder().x(10).build().unwrap())
            .build()
            .is_ok());
}
