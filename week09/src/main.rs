// Week 09: Rust basics

fn main() {
    println!("Week 09: Rust basics");
    println!("add(3, 4) = {}", add(3, 4));
    println!("multiply(3, 4) = {}", multiply(3, 4));
    println!("is_even(7) = {}", is_even(7));
    println!("max(3, 9) = {}", max(3, 9));
    println!("square(5) = {}", square(5));
    println!("reverse_string(\"hello\") = {}", reverse_string("hello"));
    println!(
        "find_max_in_vec(&[1, 5, 3]) = {:?}",
        find_max_in_vec(&[1, 5, 3])
    );
    println!(
        "count_evens(&[1, 2, 3, 4]) = {}",
        count_evens(&[1, 2, 3, 4])
    );
    println!(
        "concat_with_separator(&[\"hello\", \"world\"], \"-\") = {}",
        concat_with_separator(&["hello", "world"], "-")
    );
}

// ============================================================================
// PART 1: Basic arithmetic
// ============================================================================

fn add(a: i32, b: i32) -> i32 {
    a + b
    // In Rust, the last expression in a function is automatically returned.
    // No `return` keyword needed (though `return a + b;` also works).
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// ============================================================================
// PART 2: Boolean logic
// ============================================================================

fn is_even(n: i32) -> bool {
    n % 2 == 0
    // % is the remainder operator. If dividing by 2 leaves no remainder, it's even.
    // This works for negatives too: -4 % 2 == 0 ✓
}

// ============================================================================
// PART 3: Comparisons
// ============================================================================

fn max(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
    // if/else is an *expression* in Rust — it returns a value.
    // Both branches must return the same type.
}

// ============================================================================
// PART 4: Expressions
// ============================================================================

fn square(n: i32) -> i32 {
    n * n
}

// ============================================================================
// PART 5: String operations
// ============================================================================

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
    // .chars()   — splits the &str into individual characters
    // .rev()     — reverses the iterator
    // .collect() — gathers the characters back into a String
}

fn concat_with_separator(words: &[&str], sep: &str) -> String {
    words.join(sep)
    // Slices have a built-in .join() method — perfect for this.
    // Returns "" for an empty slice automatically.
}

// ============================================================================
// PART 6: Collections and Option
// ============================================================================

fn find_max_in_vec(numbers: &[i32]) -> Option<i32> {
    numbers
        .iter()
        .copied()
        .reduce(|a, b| if a >= b { a } else { b })
    // .iter()        — iterate over references to each number
    // .copied()      — turn &i32 references into plain i32 values
    // .reduce(...)   — fold the list down to one value using our comparison
    //                  returns None automatically if the slice is empty
}

fn count_evens(numbers: &[i32]) -> usize {
    numbers.iter().filter(|&&n| n % 2 == 0).count()
    // .filter(...)  — keep only elements where the condition is true
    // &&n           — double & because iter() gives &i32, and the closure
    //                 receives &&i32, so we pattern-match through both layers
    // .count()      — count the remaining items
    // usize is Rust's type for counts/sizes (always non-negative)
}

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }
    #[test]
    fn test_add_with_zero() {
        assert_eq!(add(0, 7), 7);
        assert_eq!(add(7, 0), 7);
    }
    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(-3, -4), -7);
    }
    #[test]
    fn test_multiply_positive() {
        assert_eq!(multiply(3, 4), 12);
    }
    #[test]
    fn test_multiply_by_zero() {
        assert_eq!(multiply(0, 99), 0);
        assert_eq!(multiply(42, 0), 0);
    }
    #[test]
    fn test_multiply_negative() {
        assert_eq!(multiply(-2, 5), -10);
        assert_eq!(multiply(-3, -3), 9);
    }
    #[test]
    fn test_is_even_true() {
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(100));
    }
    #[test]
    fn test_is_even_false() {
        assert!(!is_even(1));
        assert!(!is_even(7));
        assert!(!is_even(99));
    }
    #[test]
    fn test_is_even_negative() {
        assert!(is_even(-4));
        assert!(!is_even(-3));
    }
    #[test]
    fn test_max_first_larger() {
        assert_eq!(max(9, 3), 9);
    }
    #[test]
    fn test_max_second_larger() {
        assert_eq!(max(3, 9), 9);
    }
    #[test]
    fn test_max_equal() {
        assert_eq!(max(5, 5), 5);
    }
    #[test]
    fn test_max_negative() {
        assert_eq!(max(-1, -5), -1);
    }
    #[test]
    fn test_square_positive() {
        assert_eq!(square(5), 25);
        assert_eq!(square(1), 1);
    }
    #[test]
    fn test_square_zero() {
        assert_eq!(square(0), 0);
    }
    #[test]
    fn test_square_negative() {
        assert_eq!(square(-3), 9);
    }
    #[test]
    fn test_reverse_string_basic() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
    }
    #[test]
    fn test_reverse_string_empty() {
        assert_eq!(reverse_string(""), "");
    }
    #[test]
    fn test_reverse_string_single_char() {
        assert_eq!(reverse_string("a"), "a");
    }
    #[test]
    fn test_concat_basic() {
        assert_eq!(
            concat_with_separator(&["hello", "world"], "-"),
            "hello-world"
        );
        assert_eq!(concat_with_separator(&["a", "b", "c"], ","), "a,b,c");
    }
    #[test]
    fn test_concat_empty() {
        assert_eq!(concat_with_separator(&[], ","), "");
    }
    #[test]
    fn test_concat_single_word() {
        assert_eq!(concat_with_separator(&["hello"], "-"), "hello");
    }
    #[test]
    fn test_find_max_basic() {
        assert_eq!(find_max_in_vec(&[1, 5, 3]), Some(5));
        assert_eq!(find_max_in_vec(&[10, 2, 8, 4]), Some(10));
    }
    #[test]
    fn test_find_max_empty() {
        assert_eq!(find_max_in_vec(&[]), None);
    }
    #[test]
    fn test_find_max_negative() {
        assert_eq!(find_max_in_vec(&[-5, -1, -10]), Some(-1));
    }
    #[test]
    fn test_find_max_single() {
        assert_eq!(find_max_in_vec(&[42]), Some(42));
    }
    #[test]
    fn test_count_evens_mixed() {
        assert_eq!(count_evens(&[1, 2, 3, 4]), 2);
        assert_eq!(count_evens(&[2, 4, 6]), 3);
    }
    #[test]
    fn test_count_evens_none() {
        assert_eq!(count_evens(&[1, 3, 5]), 0);
    }
    #[test]
    fn test_count_evens_empty() {
        assert_eq!(count_evens(&[]), 0);
    }
    #[test]
    fn test_count_evens_negative() {
        assert_eq!(count_evens(&[-2, -1, 0, 1, 2]), 3);
    }
}
