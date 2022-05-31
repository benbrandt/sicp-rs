//! Exercise 1.7: The good-enough? test used in computing square roots will not
//! be very effective for finding the square roots of very small numbers. Also,
//! in real computers, arithmetic operations are almost always performed with
//! limited precision. This makes our test inadequate for very large numbers.
//! Explain these statements, with examples showing how the test fails for
//! small and large numbers. An alternative strategy for implementing
//! good-enough? is to watch how guess changes from one iteration to the next
//! and to stop when the change is a very small fraction of the guess. Design a
//! square-root procedure that uses this kind of end test. Does this work
//! better for small and large numbers?

use std::ops::Add;

/// Compare two float values accurately
fn f64_eq(x: f64, y: f64) -> bool {
    (x - y).abs() < f64::EPSILON
}

/// Return the average of 2 values, as a float
fn average<T>(x: T, y: T) -> f64
where
    T: Add<Output = T>,
    f64: From<T>,
{
    f64::from(x + y) / 2.0
}

/// Improve square root guess by averaging the guess
/// with the number (x) divided by the current guess
fn improve<T>(guess: f64, x: T) -> f64
where
    f64: From<T>,
{
    average::<f64>(guess, f64::from(x) / guess)
}

/// Find the square root of a given number using Newton's method
pub fn sqrt<T>(x: T) -> f64
where
    T: Copy,
    f64: From<T>,
{
    // Initial guess
    let mut guess = 1.0;

    // improve the guess until it is good enough with our precision
    loop {
        let next_guess = improve(guess, x);
        // If these two are equal, break out of the loop, we are done.
        if f64_eq(next_guess, guess) {
            break;
        }
        // Otherwise update guess and try again.
        guess = next_guess;
    }

    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert!(f64_eq(average(2, 1), 1.5));
        assert!(f64_eq(average(1.3333, 1.5), 1.41665));
        assert!(f64_eq(average(1.4167, 1.4118), 1.41425));
    }

    #[test]
    fn test_improve() {
        assert!(f64_eq(improve(1.0, 2), 1.5));
        assert!(f64_eq(improve(1.5, 2), 1.4166666666666665));
        assert!(f64_eq(improve(1.4166666666666665, 2), 1.4142156862745097));
    }

    #[test]
    fn test_sqrt() {
        let numbers: [f64; 5] = [
            9.0,
            0.0001,
            10000000000000.0001,
            100_000_000_000_000_000_000.0,
            0.0000000000001,
        ];

        for n in numbers {
            assert!(f64_eq(sqrt(n), n.sqrt()));
        }
    }
}
