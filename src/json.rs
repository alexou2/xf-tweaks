// a module dedicated to parsing json objects
use serde_json::{from_str, to_string_pretty, Value};

use crate::{apps, utils};
// converts a string to json
pub fn convert_to_json(json_text: &str) -> Value {
    let json_obj: Value = from_str(json_text).expect("JSON was not well-formatted");
    return json_obj;
}
// reads the file and converts it to json directly NOT TO BE USED IN CODE FINAL CODE
pub fn read_json(file_path: &str) -> Value {
    let file_content = utils::read_file(file_path).unwrap();

    return convert_to_json(&file_content);
}
// prints the json with indentations
pub fn print_json(json: &Value) {
    println!("{}", to_string_pretty(&json).unwrap());
}

// finds the command for the name of the command
pub fn find_element(element_to_find: &str) -> Vec<Value> {
    let json = apps::return_json();

    let mut command: Vec<Value> = Vec::new();

    for i in 0..json.len() {
        if json[i]["name"] == element_to_find {
            // let t: &Value = &json[i]["command"];
            command.push(json[i]["command"].clone());
            break;
        }
        // else{
        //     command.push(json!("echo \"error\""));
        // }
    }
    // let ret = command;
    return command;
}
