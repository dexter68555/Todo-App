use std::fs::File;
use std::io::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
}

//read existing tasks from json file
fn read_tasks(filename: &str) -> Result<TaskList, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let tasks: TaskList = serde_json::from_reader(reader)?;
    Ok(tasks)
}

//Display task
fn display_tasks(tasks: &TaskList) {
    let mut counter = 0;
    for task in &tasks.tasks {
        let status = if task.done { "[✓]" } else { "[ ]" };
        if task.done { counter = counter + 1 };
        println!("{} {} {}", status, task.id, task.description);
    }
    if counter > 0 {
        println!("You have {counter} pending task.");
        println!();
    }
}

//Add task
fn add_task(tasks: &mut TaskList, description: String) {
    let id = tasks.tasks.len() as u32 + 1;
    let new_task = Task {
        id,
        description,
        done: false,
    };
    tasks.tasks.push(new_task);
}

//Mark task as done
fn mark_task_done(tasks: &mut TaskList, task_id: u32) -> Result<(), &'static str> {
    for task in &mut tasks.tasks {
        if task.id == task_id {
            if !task.done {
                task.done = true;
                return Ok(());
            } else {
                return Err("Task is already marked as done.");
            }
        }
    }
    Err("Task not found.")
}

//Save task back to the json file
fn save_tasks(filename: &str, tasks: &TaskList) -> Result<(), io::Error> {
    let file = File::create(filename)?;
    serde_json::to_writer_pretty(file, tasks)?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let filename = "TaskList.json";
    let mut tasks = match read_tasks(filename) {
        Ok(task_list) => task_list,
        Err(_) => TaskList { tasks: vec![] },
    };

    loop {
        println!("ToDo List:");
        display_tasks(&tasks);
        println!("Options:");
        println!("1. Add Task to list");
        println!("2. Mark Task as Complete");
        println!("3. End");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description)?;
                add_task(&mut tasks, description.trim().to_string());
            }
            "2" => {
                println!("Enter the task ID to mark as complete:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id)?;
                let task_id = task_id.trim().parse::<u32>().unwrap();
                match mark_task_done(&mut tasks, task_id) {
                    Ok(_) => println!("Task marked as done."),
                    Err(err) => println!("{}", err),
                }
            }
            "3" => {
                println!("Ending now.");
                save_tasks(filename, &tasks)?;
                break;
            }
            _ => {
                println!("Invalid option. Please input option 1 to 3.");
                println!();
            }
        }
    }

    Ok(())
}