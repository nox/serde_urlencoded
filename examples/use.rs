use serde_urlencoded::{from_str, to_string};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct QueryParameters {
    page: u32,
    name: String,
}

fn main() {
    let params = QueryParameters {
        page: 42,
        name: "The name of the album".into(),
    };

    let actual_encoded = to_string(params.clone()).expect("Should serialize");
    let expected_encoded = "page=42&name=The+name+of+the+album";

    assert_eq!(expected_encoded, actual_encoded);
    
    let expected = from_str::<QueryParameters>(expected_encoded).expect("Should deserialize");
    assert_eq!(expected, params);
}