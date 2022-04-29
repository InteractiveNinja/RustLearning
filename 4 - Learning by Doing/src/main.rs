mod structs;
mod get_path;

fn main() {
    // structs::run();
    let full_path = get_path::get_path();
    println!("{}", full_path);
}
