pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if is_prefix(prefix, s) == true {
        return Some(&s[prefix.len()..])
    }
    None
}

pub fn is_prefix<'a>(prefix: &'a str, s: &'a str) -> bool {
    let arr = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    for ch in prefix.chars() {
        if i >= arr.len() {
            return false;
        }
        if arr[i] != ch {
            return false
        }
        i += 1;
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_prefix("ab", "abcdefghijklmnop"), true);
        assert_eq!(is_prefix("abc", "ab"), false);
        assert_eq!(is_prefix("x", "abcdefghijklmnop"), false);
        assert_eq!(delete_prefix("ab", "abcdefghijklmnop"), Some("cdefghijklmnop"));
        assert_eq!(delete_prefix("x", "abcdefghijklmnop"), None);
    }
}
