use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::{stdin, Read}, io};
use std::path::PathBuf;
use chrono::Local;
use dirs::home_dir;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: String,
    description: String,
    done: bool,
    creation_date: String,
    removed: bool
}

impl Task {
    fn new(description: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description: description.to_string(),
            done: false,
            creation_date: Local::now().to_string(),
            removed: false
        }
    }

    fn display(&self, index: usize) {
        let done = if self.done { "[x]" } else { "[ ]" };
        println!("{:>5} | {:^4} | {}", index + 1, done, self.description)
    }
}

fn load_tasks(path: &PathBuf) -> Vec<Task> {
    if let Ok(file) = File::open(path) {
        let mut file = io::BufReader::new(file);
        let mut json_content = String::new();
        if file.read_to_string(&mut json_content).is_ok() {
            return serde_json::from_str(&json_content).unwrap_or_default();
        }
    }
    Vec::new()
}

fn repeat_char(c: char, count: usize) -> String {
    std::iter::repeat(c).take(count).collect()
}

fn list_tasks(tasks: &[Task]) {
    println!("Index | Done | Description");
    println!("{} | {} | {}", repeat_char('-', 5), repeat_char('-', 4), repeat_char('-', 11));
    for (index, task) in tasks.iter().enumerate() {
        if task.removed {
            continue;
        }
        task.display(index);
    }
    println!();
}

fn write_tasks(vec: &mut Vec<Task>, path: &PathBuf) {
    if let Ok(json_string) = serde_json::to_string_pretty(vec) {
        if let Err(err) = fs::write(path, json_string) {
            eprintln!("Failed to write to file: {}", err);
        }
    } else {
        eprintln!("Failed to serialize to JSON");
    }
}

fn finish_task(tasks: &mut [Task], index: usize) {
    let update_task = tasks.get_mut(index);
    match update_task {
        Some(task) => task.done = !task.done,
        None => eprintln!("Task not found")
    };
}

fn remove_task(tasks: &mut [Task], index: usize) {
    let remove_task = tasks.get_mut(index);
    match remove_task {
        Some(task) => task.removed = true,
        None => eprintln!("Task not found")
    };
}

fn menu(tasks: &mut Vec<Task>, path: &PathBuf) {
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
        println!("4. Remove task");
        println!("X. Exit");

        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();
        println!();

        match user_input {
            "1" => {
                list_tasks(tasks);
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
                    tasks.push(new_task);
                    write_tasks(tasks, path);
                    break;
                }
            }
            "3" => {
                loop {
                    let mut index_str: String = String::new();
                    list_tasks(tasks);

                    println!("Please select a task by the index:");

                    stdin().read_line(&mut index_str).expect("Failed to read line");
                    if index_str.trim().is_empty() {
                        println!("Please select a task by index");
                        continue;
                    }

                    let index: usize = match index_str.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please insert a number");
                            continue;
                        }
                    };

                    let vec_index = index - 1;
                    if (0..tasks.len()).contains(&vec_index) {
                        finish_task(tasks, vec_index);
                        println!("Task finished successfully");
                    } else {
                        println!("Please choose an existing task");
                        continue;
                    }


                    break;
                }
                write_tasks(tasks, path);
            }
            "4" => {
                loop {
                    let mut index_str: String = String::new();
                    list_tasks(tasks);

                    println!("Please select a task by the index:");

                    stdin().read_line(&mut index_str).expect("Failed to read line");
                    if index_str.trim().is_empty() {
                        println!("Please select a task by index");
                        continue;
                    }

                    let index: usize = match index_str.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please insert a number");
                            continue;
                        }
                    };

                    let vec_index = index - 1;
                    if (0..tasks.len()).contains(&vec_index) {
                        remove_task(tasks, vec_index);
                        println!("Task removed successfully");
                    } else {
                        println!("Please choose an existing task");
                        continue;
                    }


                    break;
                }
                write_tasks(tasks, path);
            }
            "X" | "x" => break,
            _ => println!("Please choose a valid option"),
        }
    }
}

fn main() {
    // TODO: Implement a DB for Data Storage
    let mut path = home_dir().expect("Failed to get home directory").join(".argus");
    fs::create_dir_all(&path).expect("Failed to create directory");
    path = path.join("data.json");

    let mut tasks = load_tasks(&path);

    menu(&mut tasks, &path);
}
