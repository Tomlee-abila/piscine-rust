use std::ops::Mul;
use crate::scalar::Scalar;

pub mod scalar;
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.first().map_or(0, |row| row.len())
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T>
    where
        T: Clone,
    {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T>
    where
        T: Clone,
    {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Scalar,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();
        let common = self.number_of_cols();

        let mut result = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut new_row = Vec::with_capacity(cols);
            for j in 0..cols {
                let mut sum = T::zero();
                for k in 0..common {
                    let a = self.0[i][k];
                    let b = rhs.0[k][j];
                    sum = sum + a * b;
                }
                new_row.push(sum);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }
}