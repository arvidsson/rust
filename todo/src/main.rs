use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fmt;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    done: bool,
}

impl Todo {
    fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            done: false,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", if self.done { "x" } else { " " }, self.title)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut todos: Vec<Todo> = Vec::new();

    if let Ok(json) = fs::read_to_string("todos.json") {
        if let Ok(loaded_todos) = serde_json::from_str::<Vec<Todo>>(&json) {
            todos = loaded_todos;
        }
    }

    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        println!("Commands: list, add <todo>, remove <id>, do <id>");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "list" => {
            for (i, todo) in todos.iter().enumerate() {
                println!("{}: {todo}", i + 1);
            }
        }
        "add" => {
            if let Some(todo) = args.get(2) {
                todos.push(Todo::new(&todo));
            }
        }
        "remove" => {
            if let Some(id) = args.get(2) {
                if let Ok(id) = id.parse::<usize>() {
                    if id >= 1 && id < todos.len() {
                        todos.remove(id - 1);
                    }
                }
            }
        }
        "do" => {
            if let Some(id) = args.get(2) {
                if let Ok(id) = id.parse::<usize>() {
                    if id >= 1 && id < todos.len() {
                        todos[id - 1].done = true;
                    }
                }
            }
        }
        _ => println!("Unknown command!"),
    }

    if let Ok(json) = serde_json::to_string_pretty(&todos) {
        fs::write("todos.json", json).unwrap();
    }
}
