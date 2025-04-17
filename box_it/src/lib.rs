pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut numbers = Vec::new();
    for part in s.split_whitespace() {
        if part.ends_with('k') {
            let num_str = part.trim_end_matches('k');
            let num: f32 = num_str.parse().expect("Failed to parse number with 'k' suffix");
            numbers.push((num * 1000.0) as u32);
        } else {
            let num: u32 = part.parse().expect("Failed to parse number");
            numbers.push(num);
        }
    }
    Box::new(numbers)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}