#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_fails() {
        let result = 2 - 2;
        // assert_eq!(result, 4);  // FAILED
        assert_eq!(result, 0);
    }
}

fn is_even(number: i32) -> bool {
    match number % 2 {
        0 => true, // number is even
        _ => false // number is odd
    }
}

#[cfg(test)]
mod tests_is_even {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(42));
        assert!(!is_even(13));
    }
}

fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests_add_two {
    use super::add_two;

    #[test]
    fn test_with_assert() {
        assert!(add_two(2, 2) == 4);
    }

    #[test]
    fn test_with_assert_eq() {
        assert_eq!(add_two(2, 2), 4);
    }

    #[test]
    fn test_with_assert_ne() {
        assert_ne!(add_two(2, 2), 3);
    }
}

fn add_number_five(a: i32, b: i32) -> i32 {
    a + 5
}

#[cfg(test)]
mod tests_add_number_five {
    use super::add_number_five;

    #[test]
    fn test_with_assert() {
        let result = add_number_five(2, 2);
        assert!(result == 4, "Expected 4; result was {}", result);
    }

    #[test]
    fn test_with_assert_eq() {
        let result = add_number_five(2, 2);
        assert_eq!(result, 4, "Expected 4; result was {}", result);
    }

    #[test]
    fn test_with_assert_ne() {
        assert_ne!(add_number_five(2, 2), 3);
    }
}