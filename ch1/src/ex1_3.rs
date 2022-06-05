//! Exercise 1.3: Define a procedure that takes three numbers as arguments and
//! returns the sum of the squares of the two larger numbers.

use std::cmp::Ord;
use std::ops::{Add, Mul};

/// Square of a given number
fn square<T>(x: T) -> T
where
    T: Copy + Mul<Output = T>,
{
    x * x
}

/// Sum of squares of two inputs
fn sum_of_squares<T>(x: T, y: T) -> T
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    square(x) + square(y)
}

/// Return the sum of the squares of the largest 2 of 3 numbers
pub fn sum_of_largest_two_squares<T>(x: T, y: T, z: T) -> T
where
    T: Copy + Ord + Add<Output = T> + Mul<Output = T>,
{
    if x >= z && y >= z {
        sum_of_squares(x, y)
    } else if y >= x && z >= x {
        sum_of_squares(y, z)
    } else {
        sum_of_squares(x, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(1, 2), 5);
        assert_eq!(sum_of_squares(2, 3), 13);
    }

    #[test]
    fn test_sum_of_largest_two_squares() {
        let sum = sum_of_largest_two_squares(1, 2, 3);
        assert_eq!(sum, 13);
        let sum = sum_of_largest_two_squares(1, 1, 1);
        assert_eq!(sum, 2);
        let sum = sum_of_largest_two_squares(1, 2, 2);
        assert_eq!(sum, 8);
        let sum = sum_of_largest_two_squares(1, 1, 2);
        assert_eq!(sum, 5);
    }
}
