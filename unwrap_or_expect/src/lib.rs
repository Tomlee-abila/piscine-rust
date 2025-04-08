pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown =>{
            match server {
                Ok(r) => return r.to_owned(),
                Err(_) => panic!()
            }
        },
        Security::Message => {
            match server {
                Ok(s)=> return s.to_owned(),
                Err(_) => panic!("ERROR: program stops")
            }
        },
        Security::Warning => {
            match server {
                Ok(s)=> return s.to_owned(),
                Err(_) => return "WARNING: check the server".to_owned()
            }
        },
        Security::NotFound => {
            match server {
                Ok(s)=> return s.to_owned(),
                Err(e) => return format!("Not found: {}", e)
            }
        },
        Security::UnexpectedUrl => {
            match server {
                Ok(s)=> panic!("{}", s),
                Err(e) => return format!("{}", e)
            }
        }
    }
}

pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl
}