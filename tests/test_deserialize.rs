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
    #[derive(Deserialize, Default, PartialEq, Debug)]
    struct Test {
        param: Option<String>,
    }

    let result = Test { param: Some("Test".to_string()) };
    assert_eq!(serde_urlencoded::from_str("param=Test"), Ok(result));
}
