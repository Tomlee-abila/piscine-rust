pub fn rotate(input: &str, key: i8) -> String {
    let key: i32 = key as i32;
    input.chars().map(|ch|         
        if ch.is_alphabetic(){
            if ch.is_lowercase(){
                if ch as i32 + key > 122{
                    return (ch as i32 + key - 26)as u8 as char;
                }else if ch as i32 + key < 97{
                    return (ch as i32 + key + 26)as u8 as char; 
                }else{
                    return (ch as i32 + key)as u8 as char;
                }
            }else{
                if ch as i32 + key > 90{
                    return (ch as i32 + key - 26)as u8 as char;
                }else if ch as i32 + key < 65{
                    return (ch as i32 + key + 26)as u8 as char; 
                }else {
                    return (ch as i32 + key)as u8 as char;
                }
            }
        }else{
            return ch;
        }
    ).collect::<String>()
}