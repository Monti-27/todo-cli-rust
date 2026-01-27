use std::env;

use crate::{storage, todo::Todo};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let cmd = args[1].as_str();

    match cmd {
        "list" => list_todos(),
        "add" => add_todo(&args),
        "done" => mark_done(&args),
        _ => {
            println!("Unknown command: {cmd}");
            print_help();
        }
    }
}

fn list_todos() {
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

fn add_todo(args: &Vec<String>) {
    // ex: cargo run -- add "learn rust"
    if args.len() < 3 {
        println!("Usage: add \"todo text\"");
        return;
    }

    let text = args[2..].join(" "); // allows spaces without breaking

    let mut todos = storage::load();
    let id = next_id(&todos);

    todos.push(Todo::new(id, text, false));

    storage::save(&todos);

    println!("Added todo #{id}");
}

fn next_id(todos: &Vec<Todo>) -> u32 {
    // find max id and add 1
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn mark_done(args: &Vec<String>) {
    if args.len() < 3 {
        println!("usage: done <id>");
        return;
    }

    let id: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid id");
            return;
        }
    };
}

fn print_help() {
    println!("Commands:");
    println!("  add \"todo text\"");
    println!("  list");
}
