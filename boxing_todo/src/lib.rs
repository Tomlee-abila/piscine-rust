mod err;

use std::{error::Error, fs};

use json;

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
        
        let content = fs::read_to_string(path).map_err(|e| err::ReadErr {
            child_err: Box::new(e),
        })?;

        
        let json_value = json::parse(&content).map_err(|e| err::ParseErr::Malformed(Box::new(e)))?;

        
        let title = json_value["title"]
            .as_str()
            .ok_or(err::ParseErr::Malformed("Missing or invalid title".into()))?
            .to_string();

        
        let tasks_array = match &json_value["tasks"] {
            json::JsonValue::Array(arr) => arr,
            _ => return Err(Box::new(err::ParseErr::Malformed("Missing or invalid tasks array".into()))),
        };

        if tasks_array.is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }

        let tasks: Vec<Task> = tasks_array
            .iter()
            .map(|task| {
                Ok(Task {
                    id: task["id"].as_u32().ok_or(err::ParseErr::Malformed(
                        "Invalid or missing id".into(),
                    ))?,
                    description: task["description"]
                        .as_str()
                        .ok_or(err::ParseErr::Malformed(
                            "Invalid or missing description".into(),
                        ))?
                        .to_string(),
                    level: task["level"].as_u32().ok_or(err::ParseErr::Malformed(
                        "Invalid or missing level".into(),
                    ))?,
                })
            })
            .collect::<Result<Vec<Task>, Box<dyn Error>>>()?;

        Ok(TodoList { title, tasks })
    }
}