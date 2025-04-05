pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut hmap = HashMap::new();

    for c in s1.chars() {
        *hmap.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        *hmap.entry(c).or_insert(0) += 1;
    }

    hmap.values().all(|&value| value % 2 == 0)
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
