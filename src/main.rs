use std::io::{self, stdout, Write};

fn main() {
    let entrance_string: &str = "Welcome to DragonShell Version:";
    let version: &str = "0.01";
    //print both of the strings
    println!("{}", entrance_string.to_string() + " " + version);
    loop {
        print!("$ ");
        stdout().flush().unwrap();
        get_input();
    }
}

fn get_input() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    handle_commands(&input);
}

fn handle_commands(input: &str) {
    //let mut command_string: String = String::new();
    println!("You Entered:");
    print!("{}", input);
    
    /*if input.starts_with("cd ") {
    }*/
}