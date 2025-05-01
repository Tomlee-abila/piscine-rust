pub fn arrange_phrase(phrase: &str) -> String {
    let words = phrase.split_whitespace();
    let mut res: Vec<String> = vec![];

    for word in words {
        let mut num = String::from("");
        let mut hld = String::from("");

        for c in word.chars() {
            if c >= '0' && c <= '9' {
                num.push(c);
                continue
            }
            hld.push(c);
        }
        let n: usize =  num.parse().expect("error");

        if n > res.len() {
            while res.len() < n + 1 {
                res.push("".to_string());
            }
        }

        hld.push(' ');
        res[n] = hld.to_string();
    }
    let s: String = res.into_iter().collect();
    s.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(arrange_phrase("is2 Thi1s T4est 3a"), "This is a Test");
    }
}
