use std::io;

mod structs;
mod get_path;
mod create_file;

fn main() {
    // structs::run();
    // let full_path = get_path::get_path();
    // println!("{}", full_path);

    println!("Wie soll dein File heissen?");
    let mut input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut input);
    create_file::create_file(&input);


}


