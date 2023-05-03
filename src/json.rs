// a module dedicated to parsing json files
use serde_json::Value;


use crate::utils;

pub fn convert_to_json(json_text: &str) -> Value {
    let json_obj: Value = serde_json::from_str(json_text).expect("JSON was not well-formatted");
    return json_obj;
}
// reads the file and converts it to json directly
pub fn read_json(file_path: &str) -> Value {
    let file_content = utils::read_file(file_path).unwrap();

    return convert_to_json(&file_content);
}
pub fn print_json(json: &Value) {
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

