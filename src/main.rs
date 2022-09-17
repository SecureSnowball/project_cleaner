use std::{
    error::Error,
    fmt::Display,
    fs::{self, remove_dir_all},
    io,
    path::PathBuf,
    process::{self, Command},
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

enum UserInput {
    All,
    SelectedProjects(Vec<usize>),
}

#[derive(Debug, Clone)]
enum ProjectType {
    JavaScript,
    Rust,
    PHP,
    Unknown,
}

impl Default for ProjectType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProjectType::JavaScript => write!(f, "JavaScript"),
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::PHP => write!(f, "PHP"),
            ProjectType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Project {
    path: String,
    has_git: bool,
    language: ProjectType,
    cleaned: bool,
}

fn main() {
    let args = Args::parse();
    let mut projects: Vec<Project> = Vec::new();
    scan_folder(
        &args.path.into_os_string().into_string().unwrap(),
        &mut projects,
    );
    loop {
        display_projects(&projects);
        clean_handler(&mut projects);
    }
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
        TableCell::new_with_alignment("Cleaned", 1, Alignment::Left),
        TableCell::new_with_alignment("Path", 1, Alignment::Left),
    ]));
    for (index, project) in projects.iter().enumerate() {
        table.add_row(Row::new(vec![
            TableCell::new(index),
            TableCell::new(project.language.to_string()),
            TableCell::new(if project.has_git { "Yes" } else { "No" }),
            TableCell::new(if project.cleaned { "Yes" } else { "No" }),
            TableCell::new(project.path.clone()),
        ]));
    }
    println!("{}", table.render());
}

fn clean_handler(projects: &mut Vec<Project>) {
    let selected_projects = input_handler().unwrap();
    match selected_projects {
        UserInput::All => {
            for project in projects {
                clean_project(project);
            }
        }
        UserInput::SelectedProjects(project_indexes) => {
            for i in project_indexes {
                let project: Option<&mut Project> = projects.get_mut(i);
                match project {
                    Some(project) => clean_project(project),
                    None => {
                        println!("Invalid index, {}", i);
                    }
                }
            }
        }
    }
}

fn clean_project(project: &mut Project) {
    match project.language {
        ProjectType::JavaScript => {
            clean_js_project(&project.path);
            project.cleaned = true;
        }
        ProjectType::Rust => {
            clean_rust_project(&project.path);
            project.cleaned = true;
        }
        ProjectType::PHP => {
            clean_php_project(&project.path);
            project.cleaned = true;
        }
        ProjectType::Unknown => {
            eprintln!("Unable to detect project type in {}", &project.path)
        }
    }
}

fn input_handler() -> Result<UserInput, Box<dyn Error>> {
    println!(
        "Enter project number to clean.\n- 1,2 to clean both 1 and 2 project\n- a to clean all.\n- c to cancel"
    );
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim();

    if answer.to_lowercase() == String::from("c") {
        quit();
    }

    if answer.to_lowercase() == String::from("a") {
        Ok(UserInput::All)
    } else {
        let answer = format!("[{}]", answer);
        let input: Result<Vec<usize>, serde_json::Error> = serde_json::from_str(&answer[..]);
        match input {
            Ok(ids) => return Ok(UserInput::SelectedProjects(ids)),
            Err(_) => {
                println!("Invalid input");
                input_handler()
            }
        }
    }
}

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
        let mut project = Project {
            has_git,
            path: path.clone(),
            language: ProjectType::Unknown,
            ..Default::default()
        };
        if contents.contains(&String::from("vendor")) {
            project.language = ProjectType::PHP;
            projects.push(project.clone());
        }
        if contents.contains(&String::from("node_modules")) {
            project.language = ProjectType::JavaScript;
            projects.push(project.clone());
        }
        if contents.contains(&String::from("Cargo.toml")) {
            project.language = ProjectType::Rust;
            projects.push(project.clone());
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

fn clean_php_project(path: &String) {
    let delete_path = format!("{}/vendor", path);
    remove_dir_all(delete_path).unwrap();
}

fn clean_rust_project(path: &String) {
    Command::new("cargo")
        .current_dir(path)
        .args(["clean"])
        .spawn()
        .unwrap();
}

fn clean_js_project(path: &String) {
    let delete_path = format!("{}/node_modules", path);
    remove_dir_all(delete_path).unwrap();
}

fn quit() {
    println!("Bye 👋");
    process::exit(0);
}
