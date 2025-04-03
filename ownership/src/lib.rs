pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn first_subword(mut s: String) -> String {
    let mut result: String = String::new();
    let mut count = 0;
    for ch in s.chars(){
        if ((ch >= 'A' && ch <= 'Z') || ch == '_') && count != 0{
            return result;
        }
        count += 1;
        result.push(ch);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
