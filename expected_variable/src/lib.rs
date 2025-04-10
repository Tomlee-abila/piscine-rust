extern crate case;
use case::CaseExt;

pub fn expected_variable(original: &str, expected: &str) -> Option<String> {
    // Return None if not camelCase or snake_case
    if !original.is_camel_lowercase() && !is_snake_case(original) {
        return None;
    }

    // Case-insensitive edit distance
    let diff = edit_distance(&original.to_lowercase(), &expected.to_lowercase());
    let max_len = std::cmp::max(original.len(), expected.len());

    let similarity = ((max_len - diff) as f64 / max_len as f64) * 100.0;

    if similarity >= 50.0 {
        Some(format!("{}%", similarity.round() as u32))
    } else {
        None
    }
}

// Helper function for snake_case validation
fn is_snake_case(s: &str) -> bool {
    !s.is_empty()
        && s.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit())
        && !s.contains(char::is_uppercase)
        && !s.starts_with('_')
        && !s.ends_with('_')
}

// Your edit_distance function (unchanged and solid!)
pub fn edit_distance(source: &str, target: &str) -> usize {
    let w1 = source.chars().collect::<Vec<_>>();
    let w2 = target.chars().collect::<Vec<_>>();
 
    let source_length = w1.len() + 1;
    let target_length = w2.len() + 1;
 
    let mut matrix = vec![vec![0; source_length]; target_length];
 
    for i in 1..source_length { matrix[0][i] = i; }
    for j in 1..target_length { matrix[j][0] = j; }
 
    for j in 1..target_length {
        for i in 1..source_length {
            let x: usize = if w1[i-1] == w2[j-1] {
                matrix[j-1][i-1]
            } else {
                1 + std::cmp::min(
                        std::cmp::min(matrix[j][i-1], matrix[j-1][i]),
                        matrix[j-1][i-1])
            };
            matrix[j][i] = x;
        }
    }
    matrix[target_length - 1][source_length - 1]
}
