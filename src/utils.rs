// provides useful functions that are a pain in the ass to write

use crate::json;
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
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

// equivalent of typeOf()
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
