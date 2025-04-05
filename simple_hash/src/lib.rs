pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut frequency_map = HashMap::new();
    
    for word in words {
        *frequency_map.entry(word).or_insert(0) += 1;
    }
    
    frequency_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
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
