use std::{env, fs};
use std::collections::HashMap;
use std::path::Path;

struct WrappedFile {
    folder_path: String,
    file_path: String,
    folder_created: bool,
}

#[derive(Debug)]
enum Confirm {
    OKAY,
    NO,
}


fn main() {
    let files = get_files().expect("Oh no keine Argumente");
    let folder = get_pwd();
    println!("Files to wrap: {:?}, Folder where Folders will be created: '{}'", files, folder);
    let folders_to_create = folders_to_create(&folder, &files);

    println!("Folders that will be created:");
    folders_to_create.iter().for_each(|WrappedFile { file_path, folder_path, .. }| {
        println!("{} --> {}", file_path, folder_path);
    });

    let return_value = confirm_input("Hey Test 123");

    println!("{:?}", return_value);

    // let answerer = input("Do you want do proceed (y/n)");
    // let answerer_str = answerer.as_str();
    //
    // match answerer_str {
    //     "y" => {
    //         let created_folders = create_folders(&folders_to_create);
    //     }
    //     _ => {
    //         println!("Dann halt nicht :/c")
    //     }
    // }
}

/// Create Folders from Vec Array
// fn wrap_files(folders_files: &HashMap<String, String>) -> Vec<Folder> {}

/// Get Files from Arguments
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

/// Get folder which user is in
fn get_pwd() -> String {
    let path = env::current_dir().expect("Path couldnt be found");
    let path_str = path.as_os_str().to_str().expect("Could not create OS String");
    String::from(path_str)
}

/// Returns the filename from the currently running exe
fn get_exe() -> String {
    let exe_path = env::current_exe().expect("Executable couldnt be found");
    let exe_path_str = exe_path.as_os_str().to_str().expect("Could not create OS String");
    let exe_arr: Vec<&str> = exe_path_str.split("/").collect();
    let exe_str = exe_arr[exe_arr.len() - 1];
    String::from(exe_str)
}

/// Returns a list with folders paths that should be created
fn folders_to_create(folder: &String, files: &Vec<String>) -> Vec<WrappedFile> {
    let mut folder_map: Vec<WrappedFile> = Vec::new();
    for filename in files {
        let file: Vec<&str> = filename.split(".").collect();
        let folder_name = format!("{}/{}", folder, file[0]);
        folder_map.push(WrappedFile { folder_path: folder_name, file_path: filename.clone(), folder_created: false });
    }
    folder_map
}

/// User Input that takes a message, returns user input
fn confirm_input(msg: &str) -> Confirm {
    println!("{}, (y/n)", msg);
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            if !(buffer.contains("y") || buffer.contains("n")) {
                println!("Please choose between (y/n)");
                confirm_input(msg);
            };
        }
        Err(_) => {}
    };
    Confirm::NO
}
