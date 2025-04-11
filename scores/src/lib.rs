pub fn score(str: &str)-> u64{
    let mut sum: u64 = 0;
    str.chars().for_each(|c| sum += value(c));
    sum
}

fn value(ch: char)-> u64{
    match ch.to_ascii_uppercase(){
        'A'|'E'|'I'|'O'|'U'|'L'|'N'|'R'|'S'|'T' => 1,
        'D'|'G' => 2,
        'B'|'C'|'M'|'P' => 3,
        'F'|'H'|'V'|'W'|'Y' => 4,
        'K' => 5,
        'J'|'X' => 8,
        'Q'|'Z' => 10,
        _=> 0
    }
}