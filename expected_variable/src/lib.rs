use case::CaseExt;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {

    let is_snake = compared == &compared.to_snake();
    let is_camel = compared == &compared.to_camel();

    if !(is_snake || is_camel) {
        return None;
    }

    let compared_lower = compared.to_lowercase();
    let expected_lower = expected.to_lowercase();

    let distance = edit_distance(&compared_lower, &expected_lower);
    let max_len = expected.len().max(compared.len());

    let similarity = 1.0 - (distance as f64 / max_len as f64);
    let percentage = (similarity * 100.0).round() as u32;

    if similarity > 0.5 {
        Some(format!("{}%", percentage))
    } else {
        None
    }
}


pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let mut costs = vec![0; s2.len() + 1];

    for j in 0..=s2.len() {
        costs[j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        let mut last_cost = i;
        costs[0] = i + 1;

        for (j, c2) in s2.chars().enumerate() {
            let new_cost = if c1 == c2 {
                last_cost
            } else {
                1 + last_cost.min(costs[j]).min(costs[j + 1])
            };
            last_cost = costs[j + 1];
            costs[j + 1] = new_cost;
        }
    }

    costs[s2.len()]
}
