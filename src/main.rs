use std::{
    fs::{self, remove_dir_all},
    io,
    process::Command, path::PathBuf,
};

use clap::Parser;

#[derive(Debug, Parser) ]
struct Args {
    /// Path to check for projects
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    scan_folder(&args.path.into_os_string().into_string().unwrap());
}

fn scan_folder(path: &String) {
    let folder_item_iterator = fs::read_dir(path).unwrap();
    let mut contents: Vec<String> = vec![];
    for item in folder_item_iterator {
        contents.push(item.unwrap().file_name().to_str().unwrap().to_owned());
    }

    if contents.contains(&String::from(".git")) {
        if !contents.contains(&String::from(".gitignore")) {
            eprintln!("Project doesn't contain .gitignore: {}", path);
        } else {
            if contents.contains(&String::from("vendor")) {
                // It is a PHP project?
                println!("PHP Project\n{}", path);
                println!("Do you want to delete vendor folder?");
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).unwrap();
                if answer.trim() == "y" {
                    let delete_path = format!("{}/vendor", path);
                    remove_dir_all(delete_path).unwrap();
                }
            }
            if contents.contains(&String::from("node_modules")) {
                // It is a NodeJS project?
                println!("NodeJS Project\n{}", path);
                println!("Do you want to delete node_modules folder?");
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).unwrap();
                if answer.trim() == "y" {
                    let delete_path = format!("{}/node_modules", path);
                    remove_dir_all(delete_path).unwrap();
                }
            }
            if contents.contains(&String::from("Cargo.toml")) {
                // It is a Rust project?
                println!("Rust Project\n{}", path);
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).unwrap();
                if answer.trim() == "y" {
                    println!("Running cargo clean");
                    Command::new("cargo")
                        .current_dir(path)
                        .args(["clean"])
                        .spawn()
                        .unwrap();
                }
            }
        }
    } else {
        for item in fs::read_dir(path).unwrap() {
            let path = item.unwrap().path();
            if path.is_dir() {
                scan_folder(&path.into_os_string().into_string().unwrap())
            }
        }
    }
}
