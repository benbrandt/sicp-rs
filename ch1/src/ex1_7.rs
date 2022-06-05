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

/// Compare if two float values are equal (or as close as we can get)
fn f64_eq(x: f64, y: f64) -> bool {
    // Check if they are equal, or as close as we can get with floating point precision
    x == y || (x - y).abs() <= f64::EPSILON
}

/// Improve square root guess by averaging the guess
/// with the number (x) divided by the current guess
fn improve(guess: f64, x: f64) -> f64 {
    (guess + x / guess) / 2.0
}

/// Find the square root of a given number using Newton's method
pub fn sqrt<T>(x: T) -> f64
where
    f64: From<T>,
{
    // Convert x into a f64
    let x = f64::from(x);

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
    fn test_improve() {
        assert!(f64_eq(improve(1.0, 2.0), 1.5));
        assert!(f64_eq(improve(1.5, 2.0), 1.416_666_666_666_666_5));
        assert!(f64_eq(
            improve(1.416_666_666_666_666_5, 2.0),
            1.414_215_686_274_509_7
        ));
    }

    #[test]
    fn test_sqrt() {
        let numbers: [f64; 5] = [
            9.0,
            0.0001,
            10_000_000_000_000.000_1,
            100_000_000_000_000_000_000.0,
            0.000_000_000_000_1,
        ];

        for n in numbers {
            assert!(f64_eq(sqrt(n), n.sqrt()));
        }
    }
}
