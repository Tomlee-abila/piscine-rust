use std::fs;
use std::io;

pub fn open_or_create(file: &str, content: &str){
    let file = File::open(file);
    
    match file{
        Ok()=> File::write(file, content),
        Err(_)=> File::create(file, content),
        _=> unwrap()
    }
}