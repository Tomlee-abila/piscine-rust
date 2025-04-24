pub mod matrix;
pub mod scalar;

use std::ops::{Add, Sub};



#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Add for Matrix<T>
where
    T: Add<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }

        for (a_row, b_row) in self.0.iter().zip(other.0.iter()) {
            if a_row.len() != b_row.len() {
                return None;
            }
        }

        let mut result = Vec::new();
        for (a_row, b_row) in self.0.into_iter().zip(other.0.into_iter()) {
            let mut new_row = Vec::new();
            for (a, b) in a_row.into_iter().zip(b_row.into_iter()) {
                new_row.push(a + b);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }

        for (a_row, b_row) in self.0.iter().zip(other.0.iter()) {
            if a_row.len() != b_row.len() {
                return None;
            }
        }

        let mut result = Vec::new();
        for (a_row, b_row) in self.0.into_iter().zip(other.0.into_iter()) {
            let mut new_row = Vec::new();
            for (a, b) in a_row.into_iter().zip(b_row.into_iter()) {
                new_row.push(a - b);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }
}