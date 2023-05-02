// a module dedicated to parsing json files
// use serde;
// use serde_json::json;
// use serde_json::to_string;
// use serde_json::to_string_pretty;
use serde_json::Value;
use std::fs::read_to_string;

// use crate::json;

pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_buffer = read_to_string(&file_name).unwrap();
    return Ok(file_buffer);
}

pub fn convert_to_json(json_text: &str) -> Value {
    let json_obj: Value = serde_json::from_str(json_text).expect("JSON was not well-formatted");
    return json_obj;
}

pub fn read_json(file_path: &str) -> Value {
    let file_content = read_file(file_path).unwrap();

    return convert_to_json(&file_content);
}
pub fn print_json(json: &Value) {
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
