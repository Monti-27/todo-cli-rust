// a single todo item in the memory
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)] //a single todo in memory plus in json file 
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub done: bool,
}

impl Todo {
        //  this is the constructor function so we don’t repeat struct creation everywhere
    pub fn new(id: u32, text: String, done: bool) -> Self {
        Self { id, text, done : false }
    }
}