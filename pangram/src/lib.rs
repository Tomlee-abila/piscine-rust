pub fn is_pangram(s: &str) -> bool {
    let mut letters = std::collections::HashMap::new();    

    for ch in s.chars(){
        if ch.is_ascii_alphabetic(){
            letters.insert(ch.to_ascii_lowercase(), true);
        }
    }
    letters.len() == 26
}
