use std::{env};
use std::fs::File;

pub fn create_file(name : &str) {
    let p = env::current_dir().unwrap();
    let path = p.as_os_str().to_str().unwrap();
    let file_path = format!("{}/{}",path,name);
    File::create(&file_path);
    println!("Creating new File at '{}'",file_path);
}
