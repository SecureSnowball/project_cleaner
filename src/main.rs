use std::{
    fmt::Display,
    fs::{self, remove_dir_all},
    io,
    path::PathBuf,
    process::Command,
};

use clap::Parser;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

#[derive(Debug, Parser)]
struct Args {
    /// Path to check for projects
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Debug)]
enum ProjectType {
    JavaScript,
    Rust,
    PHP,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProjectType::JavaScript => write!(f, "JavaScript"),
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::PHP => write!(f, "PHP"),
        }
    }
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

fn display_projects(projects: &Vec<Project>) {
    let mut table = Table::new();
    table.style = TableStyle::extended();
    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        "Projects",
        4,
        Alignment::Center,
    )]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Sr", 1, Alignment::Right),
        TableCell::new_with_alignment("Language", 1, Alignment::Left),
        TableCell::new_with_alignment("Has Git", 1, Alignment::Left),
        TableCell::new_with_alignment("Path", 1, Alignment::Left),
    ]));
    for (index, project, ) in projects.iter().enumerate() {
        table.add_row(Row::new(vec![
            TableCell::new(index),
            TableCell::new(project.language.to_string()),
            TableCell::new(if project.has_git { "Yes" } else { "No" }),
            TableCell::new(project.path.clone()),
        ]));
    }
    println!("{}", table.render());
}
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
