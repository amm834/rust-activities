// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_lower() {
        let result = clamp(5, 10, 20);
        assert_eq!(result, 10);
    }

    #[test]
    fn clamp_upper() {
        let result = clamp(25, 10, 20);
        assert_eq!(result, 20);
    }



    #[test]
    fn concat_strings() {
        let result = concat("hello", "world");
        assert_eq!(result, "hello world");
    }


}
