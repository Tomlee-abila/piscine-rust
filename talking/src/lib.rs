pub fn talking(text: &str) -> &str {
    let text = text.trim();
    let len = text.chars().count();
    if text.is_empty(){
        return "Just say something!";
    }

    let mut answers = std::collections::HashMap::new();

    answers.insert("yell", false);      
    answers.insert("question_yell", false);
    answers.insert("question", false); 

    for (i,ch) in text.chars().enumerate(){
        if i == len-1 && ch == '?'{
            if answers["yell"]{
                answers.insert("question_yell", true);
            }else {
                answers.insert("question", true);
            }
            continue;
        }

        if ch.is_ascii_lowercase(){
            answers.insert("yell", false);
        }else{
            answers.insert("yell", true);
        }
    }

    if answers["question_yell"]{
        return "Quiet, I am thinking!";
    }else if answers["question"]{
         return "Sure.";
    }else if answers["yell"] {
        return "There is no need to yell, calm down!";
    }else{
        return "Interesting";
    }
}