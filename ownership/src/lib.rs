pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn first_subword(mut s: String) -> String {
    for (i,ch) in s.chars().enumerate(){
        if i > 0 && (ch.is_uppercase() || ch == '_'){
            s.truncate(i);
            break;
        }
    }
    s
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
