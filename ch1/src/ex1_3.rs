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

/// Takes three numbers as arguments and returns the sum of the squares of the 
/// two larger numbers.
///
/// # Examples
/// ```
/// let sum = ch1::ex1_3::sq_sum_largest(1, 2, 3);
/// assert_eq!(13, sum);
/// let sum = ch1::ex1_3::sq_sum_largest(1, 1, 1);
/// assert_eq!(2, sum);
/// let sum = ch1::ex1_3::sq_sum_largest(1, 2, 2);
/// assert_eq!(8, sum);
/// let sum = ch1::ex1_3::sq_sum_largest(1, 1, 2);
/// assert_eq!(5, sum);
/// ```
pub fn sq_sum_largest<T: Add<Output = T>>(a: T, b: T, c: T) -> T {
    a + b + c
}
