pub mod scalar;
#[derive (Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

use scalar::Scalar;

// Remove the incorrect <Item = T> bound
impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(Vec::new())
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut m = Matrix::new();
        for _ in 0..row {
            let mut r = Vec::with_capacity(col);
            for _ in 0..col {
                r.push(T::zero());
            }
            m.0.push(r);
        }
        m
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Matrix::new();
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                row.push(if j == i { T::one() } else { T::zero() });
            }
            m.0.push(row);
        }
        m
    }
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
