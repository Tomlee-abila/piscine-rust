pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn capitalize_first(input: &str) -> String {
	let mut result = String::new();

	for (i, ch) in input.chars().enumerate(){
		if i == 0{
			result.push_str(&(ch.to_ascii_uppercase().to_string()));
		}else {
			result.push(ch);
		}
	}
	result
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut in_whitespace = false;

    for (i, ch) in input.chars().enumerate() {
        if ch.is_whitespace() {
            result.push(ch);
            in_whitespace = true;
        } else {
            if in_whitespace || i == 0 {
                result.push(ch.to_ascii_uppercase());
                in_whitespace = false;
            } else {
                result.push(ch.to_ascii_lowercase());
            }
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
	let mut result: String = String::new();
	input
		.chars()
			.for_each(|ch|{
				// let mut result = ch;
				if ch.is_ascii_uppercase(){
					// result = ch.to_ascii_lowercase().next().unwrap();
					result.push(ch.to_ascii_lowercase().into())
				}else if ch.is_ascii_lowercase() {
					// result = ch.to_ascii_uppercase().next().unwrap();
					result.push(ch.to_ascii_uppercase().into())
				}else {
					result.push(ch);
				}
			});
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
