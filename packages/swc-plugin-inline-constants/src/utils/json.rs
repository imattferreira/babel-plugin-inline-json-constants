use serde_json::{from_str, Value};
use std::fs;

pub fn get_constants_json(file_path: &str) -> Value {
    let content = fs::read_to_string(file_path).unwrap();

    from_str(&content).expect("file content should be a json")
}

pub fn get_json_key<'a>(json: &'a Value, key: &'a str) -> &'a Value {
    &json[key]
}
