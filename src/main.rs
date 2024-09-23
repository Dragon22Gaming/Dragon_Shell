use std::io;

fn main() {
    println!("Welcome To Dragon's Shell!");

    get_input();
}

fn get_input() {
    let buffer: &mut String = "nothing";
    println!("*input noises*");
    io::stdin().read_line(buffer);
}