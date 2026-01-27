use std::env;

use crate::storage;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let cmd = args[1].as_str();

    match cmd {
        "list" => list_todos(),
        "add" => println!("Adding todo... (next commit)"),
        _ => {
            println!("Unknown command: {cmd}");
            print_help();
        }
    }
}

fn list_todos() {
    // load todos from file
    let todos = storage::load();

    if todos.is_empty() {
        println!("No todos found :(");
        return;
    }

    for t in todos {
        let status = if t.done { "done" } else { "not done" };
        println!("[{status}] #{} - {}", t.id, t.text);
    }
}

fn print_help() {
    println!("Commands:");
    println!("  add \"todo text\"");
    println!("  list");
}
