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
        "delete" => delete_todo(&args),
        "clear" => clear_todos(),
        "edit" => edit_todo(&args),
        "undone" => mark_undone(&args),
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

    let mut todos = storage::load();

    // find the todo and mark it done
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.mark_done();
        storage::save(&todos);
        println!("marked #{id} as done");
    } else {
        println!("todo #{id} not found");
    }
}

fn mark_undone(args: &Vec<String>) {
    if args.len() < 3 {
        println!("usage: undone <id>");
        return;
    }

    let id: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid id");
            return;
        }
    };

    let mut todos = storage::load();

    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.mark_undone();
        storage::save(&todos);
        println!("marked #{id} as not done");
    } else {
        println!("todo #{id} not found");
    }
}

fn delete_todo(args: &Vec<String>) {
    if args.len() < 3 {
        println!("usage: delete <id>");
        return;
    }

    let id: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid id");
            return;
        }
    };

    let mut todos = storage::load();
    let original_len = todos.len();

    // keep only todos that dont match the id
    todos.retain(|t| t.id != id);

    if todos.len() < original_len {
        storage::save(&todos);
        println!("deleted #{id}");
    } else {
        println!("todo #{id} not found");
    }
}

fn edit_todo(args: &Vec<String>) {
    if args.len() < 4 {
        println!("usage: edit <id> \"new text\"");
        return;
    }

    let id: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid id");
            return;
        }
    };

    let new_text = args[3..].join(" ");
    let mut todos = storage::load();

    // find the todo and update its text
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.text = new_text;
        storage::save(&todos);
        println!("updated #{id}");
    } else {
        println!("todo #{id} not found");
    }
}

fn clear_todos() {
    storage::save(&vec![]);
    println!("all todos cleared");
}

fn print_help() {
    println!("commands:");
    println!("  add \"todo text\"");
    println!("  list");
    println!("  done <id>");
    println!("  undone <id>");
    println!("  delete <id>");
    println!("  edit <id> \"new text\"");
    println!("  clear");
}
