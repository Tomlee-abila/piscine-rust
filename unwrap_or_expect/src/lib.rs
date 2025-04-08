pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match (server, security_level) {
        // Success cases
        (Ok(url), Security::Unknown) => url.to_string(),
        (Ok(url), Security::Message) => url.to_string(),
        (Ok(url), Security::Warning) => url.to_string(),
        (Ok(url), Security::NotFound) => url.to_string(),
        (Ok(url), Security::UnexpectedUrl) => panic!("{}", url),
        
        // Error cases
        (Err(_msg), Security::Unknown) => panic!(),
        (Err(_msg), Security::Message) => panic!("ERROR: program stops"),
        (Err(_msg), Security::Warning) => "WARNING: check the server".to_string(),
        (Err(msg), Security::NotFound) => format!("Not found: {}", msg),
        (Err(msg), Security::UnexpectedUrl) => msg.to_string(),
    }
}

pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl
}