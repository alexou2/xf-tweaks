// provides useful functions that are a pain in the ass to write

use serde_json::Value;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::process::Command;

// executes a bash command with arguments
pub fn run_command(command_to_run: &str) {
    let split = command_to_run.split(' ');
    let mut args: Vec<&str> = split.collect();
    let command = args[0];
    println!("{},  {:?}", command, args);
    args.remove(0);

    // the output of the command
    let output = Command::new(command)
        .args(args)
        .arg("-y")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

// equivalent of typeOf()
pub fn type_of<T>(_: &T) -> String {
    let type_of_var = std::any::type_name::<T>().to_string();
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
pub fn split_command(cmd: Vec<Value>) {
    for running_command in cmd {
        if running_command.is_array() {
            // println!("{:?}", running_command.as_array().expect("msg").len());
            for i in 0..running_command.as_array().expect("msg").len() {
                // run_command(running_command[i]);
                println!("running {}", running_command[i]);
                println!("array");
            }
        } else {
            println!("running {}", running_command);
            run_command(running_command.to_string().replace('"', "").as_str());
        }
    }
    // println!("-------------------------------\n\n")
}
