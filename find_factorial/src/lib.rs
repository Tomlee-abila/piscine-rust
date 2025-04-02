pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn factorial(num: u64) -> u64 {
    let mut result = 1;

    for number in 1..=num{
        result *= number
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
