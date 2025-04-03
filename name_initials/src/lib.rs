pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for name in names{
        let mut initial_s: Vec<String> = Vec::new();

        let names_ = name.trim().split(" ");

        for n in names_{
            if let Some(initial) = n.chars().next(){
                initial_s.push(initial.to_string()+".");
            }
        }
        result.push(initial_s.join(" "));
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
