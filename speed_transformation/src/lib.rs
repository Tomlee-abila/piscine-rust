pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h/3.6
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(km_per_hour_to_meters_per_second(100.0), 27.77777777777778);
    }
}
