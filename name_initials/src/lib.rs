pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(names.len());

    for name in names{
        let mut initial: String = String::new();
        for n in name.split_whitespace(){
            initial.push(n.chars().next().expect("error in getting string"));
            initial.push('.');
            initial.push(' ');
        }
        result.push(initial.trim_end().to_string());
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
