// a module dedicated to parsing json files
use serde_json::Value;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_buffer = read_to_string(&file_name).unwrap();
    return Ok(file_buffer);
}

pub fn convert_to_json(json_text: &str) -> Value {
    let json_obj: Value = serde_json::from_str(json_text).expect("JSON was not well-formatted");
    return json_obj;
}
// reads the file and converts it to json directly
pub fn read_json(file_path: &str) -> Value {
    let file_content = read_file(file_path).unwrap();

    return convert_to_json(&file_content);
}
pub fn print_json(json: &Value) {
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

pub fn write_file(buffer: String, target_file: &str) -> std::io::Result<()> {
    let mut file = File::create(target_file)?;
    // file.write_fmt(buffer)?;
    write!(file, "{}", buffer).expect("msg");

    Ok(())
}
