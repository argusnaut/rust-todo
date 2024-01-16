use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::{stdin, Read}, io};
use std::path::PathBuf;
use chrono::Local;
use dirs::home_dir;
use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static! {
    static ref PATH: PathBuf = {
        let home = home_dir().expect("Failed to get home directory");
        let path = home.join(".argus");
        fs::create_dir_all(&path).expect("Failed to create directory");
        path.join("data.json")
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    description: String,
    done: bool,
    creation_date: String,
}

impl Task {
    fn new(description: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description: description.to_string(),
            done: false,
            creation_date: Local::now().to_string(),
        }
    }

    fn add(self, mut tasks: Vec<Task>) -> Vec<Task> {
        tasks.push(self);
        write_tasks(&tasks);
        tasks
    }

    fn display(&self, index: usize) {
        let done = if self.done { 'x' } else { ' ' };
        println!("{}. [{}] :: {}", index + 1, done, self.description)
    }
}

fn load_tasks() -> Vec<Task> {
    if let Ok(file) = File::open(&PATH.as_mut_os_str()) {
        let mut file = io::BufReader::new(file);
        let mut json_content = String::new();
        if file.read_to_string(&mut json_content).is_ok() {
            return serde_json::from_str(&json_content).unwrap_or_default();
        }
    }
    Vec::new()
}

fn write_tasks(vec: &mut Vec<Task>) {
    if let Err(err) = fs::write(&PATH, serde_json::to_string_pretty(vec)) {
        eprintln!("Failed to write to file: {}", err);
    }
}

fn finish_task(task: &Task) {
    println!("Done: {}", task.id);
}

fn menu(tasks: &mut Vec<Task>) {
    fn repeat_char(c: char, count: usize) -> String {
        std::iter::repeat(c).take(count).collect()
    }

    println!("/{}\\", repeat_char('=', 40));
    println!("|{:^38}|", "Welcome to the Rust Todo list project");
    println!("\\{}/", repeat_char('=', 40));
    loop {
        let mut user_input = String::new();

        println!("*{}*", repeat_char('-', 40));
        println!("|{:^38}|", "Please choose an option");
        println!("*{}*\n", repeat_char('-', 40));

        println!("1. List tasks");
        println!("2. Add task");
        println!("3. Finish task");
        println!("X. Exit");

        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();

        match user_input {
            "1" => {
                for (index, task) in tasks.iter().enumerate() {
                    task.display(index);
                }
            }
            "2" => {
                let mut description = String::new();
                loop {
                    println!("Please add a description: ");
                    stdin()
                        .read_line(&mut description)
                        .expect("Failed to read the line");

                    if description.trim().is_empty() {
                        println!("Description can't be empty");
                        continue;
                    }
                    let new_task = Task::new(description.trim());
                    tasks = new_task.add(tasks.to_vec());
                    break;
                }
            }
            "3" => {
                for task in tasks.iter() {
                    finish_task(task);
                }
            }
            "X" | "x" => break,
            _ => println!("Please choose a valid option"),
        }
    }
}

fn main() {
    let mut tasks = load_tasks();
    menu(&mut tasks);
}
