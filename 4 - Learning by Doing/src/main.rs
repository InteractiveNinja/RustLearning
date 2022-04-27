mod structs;
mod get_path;

fn main() {
    // structs::run();
    let path = get_path::get_path();

    println!("{}", path);
}
