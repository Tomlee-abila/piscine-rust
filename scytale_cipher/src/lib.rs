pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut array_sipher: Vec<String> = Vec::new();
    let mut len: u32 = message.chars().count() as u32;
    let mut message: &str = &message;
    let mut result: String = String::new();

    // Step 1: Split the message into "rows" based on the size `i`
    while len > 0 {
        if len > i {
            array_sipher.push(message[0..i as usize].to_string());
            message = &message[i as usize..];
            len = message.chars().count() as u32;
        } else {
            array_sipher.push(message[0..len as usize].to_string());
            len = 0;
        }
    }

    // Step 2: Read the columns and construct the encoded message
    for j in 0..i {
        for cipher in &array_sipher {
            if cipher.chars().count() as u32 <= j {
                result.push(' '); // add a space if the column doesn't have a character
            } else {
                result.push(cipher.chars().nth(j as usize).unwrap());
            }
        }
    }

    result.trim().to_owned()
}