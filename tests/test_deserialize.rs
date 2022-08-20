use serde_derive::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct NewType<T>(T);

#[test]
fn deserialize_newtype_i32() {
    let result = vec![("field".to_owned(), NewType(11))];

    assert_eq!(serde_urlencoded::from_str("field=11"), Ok(result));
}

#[test]
fn deserialize_bytes() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(
        serde_urlencoded::from_bytes(b"first=23&last=42"),
        Ok(result)
    );
}

#[test]
fn deserialize_str() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(serde_urlencoded::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_borrowed_str() {
    let result = vec![("first", 23), ("last", 42)];

    assert_eq!(serde_urlencoded::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_reader() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(
        serde_urlencoded::from_reader(b"first=23&last=42" as &[_]),
        Ok(result)
    );
}

#[test]
fn deserialize_option() {
    let result = vec![
        ("first".to_owned(), Some(23)),
        ("last".to_owned(), Some(42)),
    ];
    assert_eq!(serde_urlencoded::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_unit() {
    assert_eq!(serde_urlencoded::from_str(""), Ok(()));
    assert_eq!(serde_urlencoded::from_str("&"), Ok(()));
    assert_eq!(serde_urlencoded::from_str("&&"), Ok(()));

    let err = serde_urlencoded::from_str::<()>("first=23").unwrap_err();
    assert_eq!(
        format!("{err}"),
        "invalid length 1, expected 0 elements in sequence"
    );
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
enum X {
    A,
    B,
    C,
}

#[test]
fn deserialize_unit_enum() {
    let result = vec![
        ("one".to_owned(), X::A),
        ("two".to_owned(), X::B),
        ("three".to_owned(), X::C),
    ];

    assert_eq!(
        serde_urlencoded::from_str("one=A&two=B&three=C"),
        Ok(result)
    );
}

#[test]
fn deserialize_unit_type() {
    assert_eq!(serde_urlencoded::from_str(""), Ok(()));
}

#[test]
fn deserialize_error_no_value() {
    let err = serde_urlencoded::from_str::<Vec<(&str, usize)>>("first&second")
        .unwrap_err();
    assert_eq!(format!("{err}"), "failed to parse value for key 'first': cannot parse integer from empty string");
}

#[test]
fn deserialize_error_vec() {
    let err = serde_urlencoded::from_str::<Vec<(&str, usize)>>(
        "first=23&second=FORTY-TWO",
    )
    .unwrap_err();
    assert_eq!(
        format!("{err}"),
        "failed to parse value for key 'second': invalid digit found in string"
    );
}

#[test]
fn deserialize_error_struct() {
    #[derive(Debug, Deserialize)]
    #[allow(unused)]
    struct Query {
        first: usize,
        second: usize,
    }

    let err = serde_urlencoded::from_str::<Query>("first=23&second=FORTY-TWO")
        .unwrap_err();
    assert_eq!(
        format!("{err}"),
        "failed to parse value for key 'second': invalid digit found in string"
    );
}
