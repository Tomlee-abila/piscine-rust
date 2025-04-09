pub struct Message{
    content: String,
    user: String,
}

impl Message{
    pub fn new() -> Self{
        Self{
            content: String::new(),
            user: String::new(),
        }
    }

    pub fn send_ms(&self) -> Option<String>{
        if self.content.len() == 0 || self.content.contains("stupid"){
            None
        }else{
            Some(self.content.clone())
        }
    }
}

pub fn check_ms(input: &str) -> Result<&str, &str> {
    let mut message = Message::new();
    message.content = input.to_string();
    match message.send_ms(){
        None => Err("ERROR: illegal"),
        Some(_) => Ok(input),
    }
}