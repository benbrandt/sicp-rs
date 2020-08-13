use std::ops::Add;

/// Sum of three arguments of the same type
///
/// # Examples
/// ```
/// let sum = ch1::ex1_3::sum_of_three(1, 2, 3);
/// assert_eq!(6, sum);
/// let sum = ch1::ex1_3::sum_of_three(1.0, 2.0, 3.0);
/// assert_eq!(6.0, sum);
/// ```
pub fn sum_of_three<T: Add<Output = T>>(a: T, b: T, c: T) -> T {
    a + b + c
}
