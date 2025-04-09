use boxing_todo::{err::{ParseErr, ReadErr}, TodoList, Task};
use json::JsonValue;
use std::{error::Error, fs::File};

#[test]
fn test_valid_todo() {
    let json_content = r#"{
        "title": "TODO LIST FOR PISCINE RUST",
        "tasks": [
            { "id": 0, "description": "do this", "level": 0 },
            { "id": 1, "description": "do that", "level": 5 }
        ]
    }"#;
    let file_path = "test_todo.json";
    File::create(file_path)
        .unwrap()
        .write_all(json_content.as_bytes())
        .unwrap();

    let result = TodoList::get_todo(file_path);
    assert!(result.is_ok());
    let todo_list = result.unwrap();
    assert_eq!(
        todo_list,
        TodoList {
            title: "TODO LIST FOR PISCINE RUST".to_string(),
            tasks: vec![
                Task {
                    id: 0,
                    description: "do this".to_string(),
                    level: 0,
                },
                Task {
                    id: 1,
                    description: "do that".to_string(),
                    level: 5,
                },
            ],
        }
    );
}

#[test]
fn test_parse_err_empty() {
    let json_content = r#"{
        "title": "TODO LIST FOR PISCINE RUST",
        "tasks": []
    }"#;
    let file_path = "test_empty_todo.json";
    File::create(file_path)
        .unwrap()
        .write_all(json_content.as_bytes())
        .unwrap();

    let result = TodoList::get_todo(file_path);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err().downcast_ref::<ParseErr>().unwrap(), ParseErr::Empty));
}

#[test]
fn test_parse_err_malformed() {
    let json_content = r#"{
        "something": ,
    }"#;
    let file_path = "test_malformed_todo.json";
    File::create(file_path)
        .unwrap()
        .write_all(json_content.as_bytes())
        .unwrap();

    let result = TodoList::get_todo(file_path);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let ParseErr::Malformed(_) = err.downcast_ref::<ParseErr>().unwrap() else {
        panic!("Expected ParseErr::Malformed");
    };
    assert!(err.source().unwrap().is::<json::Error>());
}

#[test]
fn test_read_err() {
    let result = TodoList::get_todo("non_existent_file.json");
    assert!(result.is_err());
    let ReadErr { child_err } = result.unwrap_err().downcast_ref::<ReadErr>().unwrap() else {
        panic!("Expected ReadErr");
    };
    assert!(child_err.is::<std::io::Error>());
}