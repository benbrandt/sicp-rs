//! Exercise 1.11. A function f is defined by the rule that
//! f(n) = n if n<3 and f(n) = f(n - 1) + 2f(n - 2) + 3f(n - 3) if n >= 3.
//!
//! Write a procedure that computes f by means of a recursive process.
//! Write a procedure that computes f by means of an iterative process.

/// f(n) = n if n<3 and f(n) = f(n - 1) + 2f(n - 2) + 3f(n - 3) if n>= 3
#[must_use]
pub fn f_recursive(n: u32) -> u32 {
    if n < 3 {
        n
    } else {
        f_recursive(n - 1) + (2 * f_recursive(n - 2)) + (3 * f_recursive(n - 3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_of_n_below_3() {
        for n in 0..3 {
            assert_eq!(f_recursive(n), n);
        }
    }

    #[test]
    fn f_of_4() {
        assert_eq!(f_recursive(4), ((2 + 2) + 2 * 2 + 3));
    }

    #[test]
    fn f_of_5() {
        assert_eq!(f_recursive(5), 11 + 2 * 4 + 3 * 2);
    }

    #[test]
    fn f_of_6() {
        assert_eq!(f_recursive(6), 25 + 2 * 11 + 3 * 4);
    }
}
