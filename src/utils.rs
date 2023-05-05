// provides useful functions that are a pain in the ass to write

use crate::json;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn run_command(command_to_run: &str) {
    let split = command_to_run.split(' ');
    let mut args: Vec<&str> = split.collect();
    let command = args[0];
    println!("{},  {:?}", command, args);
    args.remove(0);


    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

// pub fn convert_to_struct() {
//     let data = &json::read_json("commands.json")["applications"];
//     // println!("{}", data);
//     let arr = data.as_array().unwrap();
//     let mut buffer: Vec<String> = Vec::new();

//     for i in arr {

//         let str = format!(
//             "let {} = install_commands{{
//             name:{},
//             command:vec![{}],
//             description:{},
//             needs_sudo:{},
//             app_type:{},
//         }};
//         {}.push(&{});\n",
//         // values for the struct
//             i["name"]
//                 .to_string()
//                 .replace('"', "")
//                 .replace("-", "_")
//                 .replace(" ", ""),
//             i["name"],
//             i["command"].to_string().replace('[', "").replace(']', ""),
//             i["description"],
//             i["needs_sudo"],
//             i["type"],
//             i["type"].to_string().replace('"', ""),
//             i["name"]
//                 .to_string()
//                 .replace('"', "")
//                 .replace("-", "_")
//                 .replace(" ", "")
//         );
//         buffer.push(str);
//     }

//     write_file(buffer.join("\n"), "result.struct");
// }

pub fn type_of<T>(_: &T) -> String {
    let type_of_var = format!("{}", std::any::type_name::<T>());
    println!("{}", &type_of_var);
    return type_of_var;
}

pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_buffer = read_to_string(&file_name).unwrap();
    return Ok(file_buffer);
}

pub fn write_file(buffer: String, target_file: &str) -> std::io::Result<()> {
    let mut file = File::create(target_file)?;
    write!(file, "{}", buffer).expect("msg");

    Ok(())
}
