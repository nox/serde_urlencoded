extern crate serde_urlencoded;
#[macro_use]
extern crate serde_derive;

#[test]
fn serialize_option_map_int() {
    let params = &[("first", Some(23)), ("middle", None), ("last", Some(42))];

    assert_eq!(serde_urlencoded::to_string(params),
               Ok("first=23&last=42".to_owned()));
}

#[test]
fn serialize_option_map_string() {
    let params =
        &[("first", Some("hello")), ("middle", None), ("last", Some("world"))];

    assert_eq!(serde_urlencoded::to_string(params),
               Ok("first=hello&last=world".to_owned()));
}

#[test]
fn serialize_option_map_bool() {
    let params = &[("one", Some(true)), ("two", Some(false))];

    assert_eq!(serde_urlencoded::to_string(params),
               Ok("one=true&two=false".to_owned()));
}

#[test]
fn serialize_map_bool() {
    let params = &[("one", true), ("two", false)];

    assert_eq!(serde_urlencoded::to_string(params),
               Ok("one=true&two=false".to_owned()));
}

#[derive(Serialize)]
enum X {
    A,
    B,
    C,
}

#[test]
fn serialize_unit_enum() {
    let params = &[("one", X::A), ("two", X::B), ("three", X::C)];
    assert_eq!(serde_urlencoded::to_string(params),
               Ok("one=A&two=B&three=C".to_owned()));
}

#[test]
fn serialize_sequence() {
    let params = &[("first", Some(23)), ("first", None), ("first", Some(34)), ("last", Some(42))];

    assert_eq!(serde_urlencoded::to_string(params),
               Ok("first=23&first=34&last=42".to_owned()));
}

#[derive(Serialize)]
struct User {
    first_name: String,
    last_name: String,
}

#[test]
fn serialize_struct() {
    let params = User {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
    };

    assert_eq!(serde_urlencoded::to_string(params),
               Ok("first_name=John&last_name=Doe".to_owned()));
}

#[derive(Serialize)]
struct Customer {
    first_name: String,
    last_name: String,
    emails: Vec<String>,
}

#[test]
fn serialize_struct_with_vec() {
    let params = Customer {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        emails: vec!["john@example.com".to_owned(), "doe@example.com".to_owned()],
    };

    assert_eq!(serde_urlencoded::to_string(params),
               Ok("first_name=John&last_name=Doe&emails=john%40example.com&emails=doe%40example.com".to_owned()));
}
