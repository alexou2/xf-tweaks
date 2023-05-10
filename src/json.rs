// a module dedicated to parsing json objects
use serde_json::{from_str, Value};

use crate::{apps, utils};
// converts a string to json
pub fn convert_to_json(json_text: &str) -> Value {
    let json_obj: Value = serde_json::from_str(json_text).expect("JSON was not well-formatted");
    return json_obj;
}
// reads the file and converts it to json directly NOT TO BE USED IN CODE FINAL CODE
pub fn read_json(file_path: &str) -> Value {
    let file_content = utils::read_file(file_path).unwrap();

    return convert_to_json(&file_content);
}
// prints the json with indentations
pub fn print_json(json: &Value) {
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

// finds the command for the name of the command
pub fn find_element(element_to_find: &str) -> String {
    let json = apps::return_json();

    let mut command: String = "".to_string();
    for i in 0..json.len() {
        if json[i]["name"] == element_to_find {
            command = json[i]["command"].to_string().replace('"', "");
            break;
        }
    }
    println!("{:?}", command);
    return command;
}
