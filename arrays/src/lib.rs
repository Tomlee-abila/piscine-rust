pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn sum(a: &[i32; 32]) -> i32 {
	a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
	let arr: [i32; 32] = [10; 32];
    arr
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
