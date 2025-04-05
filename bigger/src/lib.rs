pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
	let mut big: i32 = 0;

	for hmap in h{
		if hmap.1 > big{
			big = hmap.1;
		}
	}
	big
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
