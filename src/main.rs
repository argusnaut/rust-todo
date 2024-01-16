use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{stdin, Read},
    path::Path,
};
use chrono::Local;
use uuid::Uuid;

const PATH: &str = "C:\\Users\\vitor37806\\.argus\\data.json";

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
        let done = match self.done {
            true => 'x',
            false => ' ',
        };

        println!("{}. [{}] :: {}", index, done, self.description)
    }
}

fn load_tasks() -> Vec<Task> {
    let file_path = Path::new(&PATH);
    let mut tasks: Vec<Task> = vec![];
    if file_path.exists() {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut json_content = String::new();
        file.read_to_string(&mut json_content)
            .expect("Failed to read from file");

        tasks = serde_json::from_str(&json_content).expect("Failed to deserialize from JSON");
    } else {
        write_tasks(&tasks)
    }
    tasks
}

fn write_tasks(vec: &Vec<Task>) {
    let json_string = serde_json::to_string_pretty(vec).expect("Failed to serialize to JSON");
    std::fs::write("data.json", json_string).expect("Failed to write to file");
}

fn main() {
    let mut tasks = load_tasks();
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
        println!("3. Exit");

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

                    if description.is_empty() {
                        println!("Description can't be empty");
                        continue;
                    }
                    break;
                }
                let new_task = Task::new(description.trim());
                tasks = new_task.add(tasks);
            }
            "3" => break,
            _ => println!("Please choose a valid option"),
        }
    }
}
