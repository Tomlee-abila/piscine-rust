pub fn pig_latin(text: &str) -> String {
    let mut result: String = String::from("");
    let mut third = 0;

    for (i, ch) in text.char_indices() {
        if ch.is_ascii_alphabetic() {
            if i == 0 && is_vowel(ch) {
                return format!("{}", text);
            }
            if is_vowel(ch) {
                if third > 0 && ch.to_ascii_lowercase() == 'u' && third + 1 == i {
                    result = format!("{}{}", result, ch);
                    if i + 1 < text.chars().count() {
                        return format!("{}{}ay", &text[i + 1..], result);
                    }
                    return format!("{}{}ay", &text[i..], result);
                } else {
                    return format!("{}{}ay", &text[i..], result);
                }
            }
            if !is_vowel(ch) {
                if ch.to_ascii_lowercase() == 'q' {
                    third = i;
                }
                result = format!("{}{}", result, ch);
            }
        }
    }
    format!("")
}

fn is_vowel(ch: char) -> bool {
    matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
