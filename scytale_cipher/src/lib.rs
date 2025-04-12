fn scytale_cipher(message: String, i: u32) -> String {
    let mut rows: Vec<String> = Vec::new();
    let len = message.chars().count() as u32;
    let num_rows = (len + i - 1) / i; 

    
    for row in 0..num_rows {
        let start = (row * i) as usize;
        let end = ((row + 1) * i) as usize;
        let row_message = &message[start..end.min(len as usize)]; 
        rows.push(row_message.to_string());
    }

    let mut result = String::new();
    for col in 0..i {
        for row in 0..num_rows {
            if let Some(ch) = rows.get(row as usize).and_then(|r| r.chars().nth(col as usize)) {
                result.push(ch);
            }
        }
    }

    result
}