use std::io;

fn main() {
    let entrance_string: &str = "Welcome to DragonShell Version:";
    let version: &str = "0.01";
    //print both of the strings
    println!("{}", entrance_string.to_owned().to_string() + " " + version);
    get_input();
}

fn get_input() {
    let mut input = String::new();
    println!("*input noises*");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{}", input);
}