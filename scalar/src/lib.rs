pub fn sum(a: u8, b: u8) -> u8 {
    a+b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a-b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a*b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a/b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a%b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = sum(2, 2);
        assert_eq!(sum(2, 2), 4);
        assert_eq!(diff(6, 2), 4);
        assert_eq!(pro(6, 2), 12);
        assert_eq!(quo(6.0, 2.0), 3.0);
        assert_eq!(rem(6.0, 2.0), 0.0);
    }
}
