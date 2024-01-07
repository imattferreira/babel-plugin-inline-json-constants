use std::fs;

pub fn get_constants_json(file_path: &str) -> serde_json::Value {
    let content = fs::read_to_string(file_path).unwrap();

    serde_json::from_str(&content).expect("file content should be a json")
}

pub fn get_json_key<'a>(json: &'a serde_json::Value, key: &'a str) -> &'a serde_json::Value {
    &json[key]
}
