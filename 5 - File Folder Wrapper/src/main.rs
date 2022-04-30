use std::{env, fs};

fn main() {
    let files = get_files().expect("Oh no keine Argumente");
    let folder = get_pwd();
    println!("Files to wrap: {:?}, Folder where Folders will be created: '{}'", files, folder);
    let folders_to_create = folders_to_create(&folder, &files);

    println!("Folders that will be created:");
    folders_to_create.iter().for_each(|name| {
        println!("{}", name);
    });

    let answerer = input("Do you want do proceed (y/n)");
    let answerer_str = answerer.as_str();

    match answerer_str {
        "y" => {
            println!("Noice");
        }
        _ => {
            println!("Dann halt nicht :/c")
        }
    }
}


fn get_files() -> Result<Vec<String>, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Not Files selected");
    }
    let names = &args[1..args.len()];
    let exe_file = get_exe();
    let mut files: Vec<String> = Vec::new();
    for name in names {
        if name.contains(&exe_file) {
            continue;
        }
        let is_file = fs::metadata(name).expect("File doesn't exist").is_file();
        if is_file {
            files.push(name.clone());
        }
    }
    Ok(Vec::from(files))
}

fn get_pwd() -> String {
    let path = env::current_dir().expect("Path couldnt be found");
    let path_str = path.as_os_str().to_str().expect("Could not create OS String");
    String::from(path_str)
}

fn get_exe() -> String {
    let exe_path = env::current_exe().expect("Executable couldnt be found");
    let exe_path_str = exe_path.as_os_str().to_str().expect("Could not create OS String");
    let exe_arr: Vec<&str> = exe_path_str.split("/").collect();
    let exe_str = exe_arr[exe_arr.len()-1];
    String::from(exe_str)
}

fn folders_to_create(folder: &String, files: &Vec<String>) -> Vec<String> {
    let mut to_create = Vec::new();

    for filename in files {
        let file: Vec<&str> = filename.split(".").collect();
        let folder_name = format!("{}/{}", folder, file[0]);
        to_create.push(folder_name);
    }
    to_create
}

fn input(msg: &str) -> String {
    println!("Input: {}", msg);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    buffer.trim_end().to_owned()
}
