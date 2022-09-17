use std::{
    fs::{self, remove_dir_all},
    io,
    path::PathBuf,
    process::Command,
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Path to check for projects
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

enum ProjectType {
    JavaScript,
    Rust,
    PHP,
}

struct Project {
    path: String,
    has_git: bool,
    language: ProjectType,
}

fn main() {
    let args = Args::parse();
    let mut projects: Vec<Project> = Vec::new();
    scan_folder(
        &args.path.into_os_string().into_string().unwrap(),
        &mut projects,
    );
    display_projects(&projects);
    clean_projects(&projects);
}

fn display_projects(projects: &Vec<Project>) {}
fn clean_projects(projects: &Vec<Project>) {}

fn scan_folder(path: &String, projects: &mut Vec<Project>) {
    let folder_item_iterator = fs::read_dir(path).unwrap();
    let mut contents: Vec<String> = vec![];
    for item in folder_item_iterator {
        contents.push(item.unwrap().file_name().to_str().unwrap().to_owned());
    }

    if contents.contains(&String::from(".git")) {
        let mut has_git = true;
        if !contents.contains(&String::from(".gitignore")) {
            has_git = false;
            eprintln!("Project doesn't contain .gitignore: {}", path);
        }
        if contents.contains(&String::from("vendor")) {
            // handle_php_project(path);
            projects.push(Project {
                has_git,
                path: path.clone(),
                language: ProjectType::PHP,
            })
        }
        if contents.contains(&String::from("node_modules")) {
            projects.push(Project {
                has_git,
                path: path.clone(),
                language: ProjectType::JavaScript,
            })
            // handle_js_project(path);
        }
        if contents.contains(&String::from("Cargo.toml")) {
            // handle_rust_project(path);
            projects.push(Project {
                has_git,
                path: path.clone(),
                language: ProjectType::Rust,
            })
        }
    } else {
        for item in fs::read_dir(path).unwrap() {
            let path = item.unwrap().path();
            if path.is_dir() {
                scan_folder(&path.into_os_string().into_string().unwrap(), projects);
            }
        }
    }
}

fn handle_php_project(path: &String) {
    println!("PHP Project\n{}", path);
    println!("Do you want to delete vendor folder?");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    if answer.trim() == "y" {
        let delete_path = format!("{}/vendor", path);
        remove_dir_all(delete_path).unwrap();
    }
}

fn handle_rust_project(path: &String) {
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

fn handle_js_project(path: &String) {
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
