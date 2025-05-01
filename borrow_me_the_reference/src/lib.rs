pub fn delete_and_backspace(s: &mut String) {
    let mut count = 0;
    let mut result = String::new(); 

    for c in s.chars() {
        if c == '-' {
            if result.len() >= 2 {
                result.truncate(result.len() - 1);
            }
            continue;
        } else if c == '+' {
            count += 1;
            continue;
        }

        if count > 0 {
            count -= 1;
            continue;
        }

        result.push(c);
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for word in v {
        let mut hld = String::from("");
        let mut hld1 = String::from("");
        let mut op = String::from("");
        let mut check = false;

        for c in word.chars() {
            if c == '-' && hld.len() > 0 {
                op = "-".to_string();
                check = true;
                continue
            } else if c == '+' && hld.len() > 0 {
                op = "+".to_string();
                check = true;
                continue
            }

            if check {
                hld1.push(c);
                continue
            } else {
                hld.push(c);
                continue
            }
        }

        let n1: i32 = hld.parse().expect("error");
        let n2: i32 = hld1.parse().expect("error");

        if op == "+".to_string() {
            *word = (n1 + n2).to_string();
        } else if op == "-".to_string() {
            *word = (n1 - n2).to_string();
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        delete_and_backspace(&mut a);
        let mut b = [
        "2+2".to_owned(),
        "3+2".to_owned(),
        "10-3".to_owned(),
        "5+5".to_owned(),
        ];
        do_operations(&mut b);

        assert_eq!(a, "borrow".to_owned());
        assert_eq!(b, ["4".to_string(), "5".to_string(), "7".to_string(), "10".to_string()]);
    }
}
