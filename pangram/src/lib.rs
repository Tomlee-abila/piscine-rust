pub fn is_pangram(s: &str) -> bool {
    let mut result: Vec<char> = Vec::new();

    for ch in s.chars(){
        if !result.contains(&ch.to_ascii_lowercase()) && ch.is_ascii_alphabetic(){
            result.push(ch.to_ascii_lowercase());
        }
    }
    if result.len() == 26{
        return  true;
    }
    false
}