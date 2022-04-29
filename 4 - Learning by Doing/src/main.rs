mod structs;
mod get_path;

fn main() {
    // structs::run();
    let path = get_path::get_path();
    let full_path = {
        let path: Vec<&str> = path.split("/").collect();
        path[0]
    };
    println!("{}", full_path);
}
