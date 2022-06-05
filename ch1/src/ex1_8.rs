//! Exercise 1.8: Newton’s method for cube roots is based on the fact that if y
//! is an approximation to the cube root of x, then a better approximation is
//! given by the value
//!
//! (x / y^2 + 2y) / 3
//!
//! Use this formula to implement a cube-root procedure analogous to the
//! square-root procedure.

use crate::ex1_7::f64_eq;

/// Improve cube root guess using Newton's method for approximating a better guess
fn improve(guess: f64, x: f64) -> f64 {
    // Should be the same as: (x / (guess * guess) + 2.0 * guess) / 3.0
    // https://rust-lang.github.io/rust-clippy/master/index.html#suboptimal_flops
    2.0f64.mul_add(guess, x / (guess * guess)) / 3.0
}

/// Find the cube root of a given number using Newton's method
pub fn cbrt<T>(x: T) -> f64
where
    f64: From<T>,
{
    // Convert x into a f64
    let x = f64::from(x);

    // Initial guess
    let mut guess = 1.1;

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
    fn test_sqrt() {
        let numbers: [f64; 5] = [5.0, -2.0, 27.0, 0.0, 100_000_000_000_000.000_1];

        for n in numbers {
            // Not as accurate as built-in, so can't use our `f64_eq` function.
            assert!((n.cbrt() - cbrt(n)).abs() < 1e-10);
        }
    }
}
