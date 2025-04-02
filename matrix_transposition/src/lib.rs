pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix((i32, i32), (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
   Matrix((m.0.0, m.1.0), (m.0.1, m.1.1))
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
