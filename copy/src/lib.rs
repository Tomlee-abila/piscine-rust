pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let expo: Vec<String> = a.split_whitespace()
        .map(
            |x|
            x.parse::<f64>()
                .expect("error converting to f64")
                    .exp().to_string()
        ).collect();

    (a, expo.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let nl_nums: Vec<f64> = b.iter()
        .map(|num| 
            (num.abs() as f64).ln()).collect();
    (b, nl_nums)
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
