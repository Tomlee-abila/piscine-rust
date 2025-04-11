pub fn num_to_ordinal(x: u32) -> String {
    format!("{}{}",x, ordinal(x))
}

fn ordinal(num: u32) -> String {
    match num % 100 {
        11 | 12 | 13 => "th".to_string(),
        _ => match num % 10 {
            1 => "st".to_string(),
            2 => "nd".to_string(),
            3 => "rd".to_string(),
            _ => "th".to_string(),
        },
    }
}