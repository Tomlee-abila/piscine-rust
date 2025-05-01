mod err;

use std::{error::Error, fs};
use json::JsonValue;
pub use err::{ ParseErr, ReadErr };

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(err::ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;
        
        let parsed: JsonValue = json::parse(&content)
            .map_err(|e| Box::new(err::ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;
        
        let title = parsed["title"].as_str()
            .ok_or_else(|| Box::new(err::ParseErr::Empty) as Box<dyn Error>)?
            .to_string();
    
        let tasks_val = &parsed["tasks"];
        if !tasks_val.is_array() {
            return Err(Box::new(err::ParseErr::Empty));
        }
        
        let mut tasks = Vec::new();
        for task_val in tasks_val.members() {
            let id = task_val["id"].as_u32()
                .ok_or_else(|| Box::new(err::ParseErr::Empty) as Box<dyn Error>)?;
            let description = task_val["description"].as_str()
                .ok_or_else(|| Box::new(err::ParseErr::Empty) as Box<dyn Error>)?
                .to_string();
            let level = task_val["level"].as_u32()
                .ok_or_else(|| Box::new(err::ParseErr::Empty) as Box<dyn Error>)?;
            tasks.push(Task { id, description, level });
        }
        
        if tasks.is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }
        
        Ok(TodoList { title, tasks })
    }
}
