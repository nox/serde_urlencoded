extern crate serde_urlencoded;
#[macro_use]
extern crate serde_derive;

#[test]
fn deserialize_bytes() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(serde_urlencoded::from_bytes(b"first=23&last=42"),
               Ok(result));
}

#[test]
fn deserialize_str() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(serde_urlencoded::from_str("first=23&last=42"),
               Ok(result));
}

#[test]
fn deserialize_reader() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(serde_urlencoded::from_reader(b"first=23&last=42" as &[_]),
               Ok(result));
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
    assert!(serde_urlencoded::from_str::<()>("first=23").is_err());
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
        ("three".to_owned(), X::C)
    ];

    assert_eq!(serde_urlencoded::from_str("one=A&two=B&three=C"), Ok(result));
}

#[test]
fn deserialize_sequence() {
    let result = vec![("first".to_owned(), Some(23)), ("first".to_owned(), Some(34)), ("last".to_owned(), Some(42))];

    assert_eq!(serde_urlencoded::from_bytes(b"first=23&first=34&last=42"),
               Ok(result));
}

#[test]
fn deserialize_sequence_quoted_key() {
    let result = vec![("first".to_owned(), Some(23)), ("first".to_owned(), Some(34)), ("last".to_owned(), Some(42))];

    assert_eq!(serde_urlencoded::from_bytes(b"first=23&f%69rst=34&last=42"),
               Ok(result));
}

#[derive(Deserialize, PartialEq, Debug)]
struct User {
    first_name: String,
    last_name: String,
}

#[test]
fn deserialize_struct() {
    let result = User {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
    };

    assert_eq!(serde_urlencoded::from_bytes(b"first_name=John&last_name=Doe"),
               Ok(result));
}

#[derive(Deserialize, PartialEq, Debug)]
struct Customer {
    first_name: String,
    last_name: String,
    emails: Vec<String>,
}

#[test]
fn deserialize_struct_with_vec() {
    let result = Customer {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        emails: vec!["john@example.com".to_owned(), "doe@example.com".to_owned()],
    };

    assert_eq!(serde_urlencoded::from_bytes(b"first_name=John&last_name=Doe&emails=john%40example.com&emails=doe%40example.com"),
               Ok(result));
}
