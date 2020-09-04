use std::cmp::Ord;
use std::ops::{Add, Mul};

/// Square of a given number
fn square<T: Copy + Mul<Output = T>>(x: T) -> T {
    x * x
}

/// Sum of squares of two inputs
fn sum_squares<T: Copy + Add<Output = T> + Mul<Output = T>>(x: T, y: T) -> T {
    square(x) + square(y)
}

pub fn sum_squares_largest<T: Copy + Ord + Add<Output = T> + Mul<Output = T>>(
    a: T,
    b: T,
    c: T,
) -> T {
    if a >= c && b >= c {
        sum_squares(a, b)
    } else if b >= a && c >= a {
        sum_squares(b, c)
    } else {
        sum_squares(a, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_square() {
        assert_eq!(super::square(2), 4);
    }

    #[test]
    fn test_sum_squares() {
        assert_eq!(super::sum_squares(1, 2), 5);
        assert_eq!(super::sum_squares(2, 3), 13);
    }

    #[test]
    fn test_sum_squares_largest() {
        let sum = super::sum_squares_largest(1, 2, 3);
        assert_eq!(sum, 13);
        let sum = super::sum_squares_largest(1, 1, 1);
        assert_eq!(sum, 2);
        let sum = super::sum_squares_largest(1, 2, 2);
        assert_eq!(sum, 8);
        let sum = super::sum_squares_largest(1, 1, 2);
        assert_eq!(sum, 5);
    }
}
