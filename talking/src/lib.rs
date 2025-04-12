pub fn talking(text: &str) -> &str {
    let text = text.trim();

    if text.is_empty(){
        return "Just say something!";
    }

    let question = text.ends_with('?');
    let has_letters = text.chars().any(|c| c.is_alphabetic());
    let yell = has_letters && text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

    match (yell, question) {
        (true, false) => return "There is no need to yell, calm down!",
        (true, true) => return "Quiet, I am thinking!",
        (false, true) => return "Sure.",
        _=> return "Interesting"
    }
}