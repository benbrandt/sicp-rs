//! The following pattern of numbers is called Pascal's triangle.
//!
//! ```md
//!     1
//!    1 1
//!   1 2 1
//!  1 3 3 1
//! 1 4 6 4 1
//! ```
//!
//! The numbers at the edge of the triangle are all 1, and each number inside
//! the triangle is the sum of the two numbers above it. Write a procedure that
//! computes elements of Pascal's triangle by means of a recursive process.

/// Compute the value of a given element in a Pascal Triangle
#[must_use]
pub fn pascal(row: usize, col: usize) -> usize {
    if col == 0 || col == row {
        1
    } else {
        pascal(row - 1, col - 1) + pascal(row - 1, col)
    }
}

/// Compute the entire Pascal triangle up to a given row
#[must_use]
pub fn pascal_triangle(rows: usize) -> Vec<Vec<usize>> {
    let mut triangle = Vec::with_capacity(rows);

    // Build each row
    for row in 1..=rows {
        let mut current_row = Vec::with_capacity(row + 1);
        current_row.push(1); // First element

        // Calculate next elements
        let mut val = 1;
        for col in 1..row {
            val *= row - col;
            val /= col;
            current_row.push(val);
        }

        triangle.push(current_row);
    }

    triangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal() {
        assert_eq!(pascal(0, 0), 1);
        assert_eq!(pascal(1, 0), 1);
        assert_eq!(pascal(1, 1), 1);
        assert_eq!(pascal(2, 1), 2);
        assert_eq!(pascal(4, 2), 6);
        assert_eq!(pascal(7, 4), 35);
    }

    #[test]
    fn test_pascal_triangle() {
        let triangle = pascal_triangle(8);
        assert_eq!(triangle[0], vec![1]);
        assert_eq!(triangle[1], vec![1, 1]);
        assert_eq!(triangle[2], vec![1, 2, 1]);
        assert_eq!(triangle[3], vec![1, 3, 3, 1]);
        assert_eq!(triangle[4], vec![1, 4, 6, 4, 1]);
        assert_eq!(triangle[5], vec![1, 5, 10, 10, 5, 1]);
        assert_eq!(triangle[6], vec![1, 6, 15, 20, 15, 6, 1]);
        assert_eq!(triangle[7], vec![1, 7, 21, 35, 35, 21, 7, 1]);
    }
}
