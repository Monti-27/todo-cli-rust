use std::fs;

use crate::todo::Todo;

// this is the file where we store todos
const FILE_PATH: &str = "todos.json";

pub fn load() -> Vec<Todo> {
    // If file not found => return empty list
    let content = fs::read_to_string(FILE_PATH);
    match content {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

pub fn save(todos: &Vec<Todo>) {
    // converting todos into pretty JSON
    let json = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");

    // writing the JSON into file
    fs::write(FILE_PATH, json).expect("Failed to save todos");
}