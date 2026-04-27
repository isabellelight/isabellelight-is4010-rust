#![allow(dead_code)]
use std::fmt;

// ============================================================================
// GENERIC STACK
// ============================================================================

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

// ============================================================================
// DISPLAY TRAIT
// ============================================================================

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

// ============================================================================
// ITERATOR — separate struct so IntoIterator doesn't conflict
// ============================================================================

struct StackIterator<T> {
    stack: Stack<T>,
}

impl<T> Iterator for StackIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator { stack: self }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Lab 12: Generic Stack Implementation\n");

    println!("=== Integer Stack ===");
    let mut int_stack = Stack::new();
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);
    println!("Stack: {}", int_stack);
    println!("Popped: {:?}", int_stack.pop());
    println!("Peek: {:?}", int_stack.peek());
    println!();

    println!("=== String Stack ===");
    let mut string_stack: Stack<String> = Stack::new();
    string_stack.push(String::from("Rust"));
    string_stack.push(String::from("is"));
    string_stack.push(String::from("awesome"));
    println!("Stack: {}", string_stack);

    println!("\nIterating (LIFO order):");
    for item in string_stack {
        println!("  {}", item);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_push_increases_length() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_pop_returns_last_pushed() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek_without_removing() {
        let mut stack = Stack::new();
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_peek_empty_stack() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_with_strings() {
        let mut stack = Stack::new();
        stack.push(String::from("hello"));
        stack.push(String::from("world"));
        assert_eq!(stack.pop(), Some(String::from("world")));
    }

    #[test]
    fn test_push_pop_sequence() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn test_display_format() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(format!("{}", stack), "[1, 2, 3]");
    }

    #[test]
    fn test_display_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(format!("{}", stack), "[]");
    }

    #[test]
    fn test_display_strings() {
        let mut stack = Stack::new();
        stack.push("hello");
        stack.push("world");
        assert_eq!(format!("{}", stack), "[hello, world]");
    }

    #[test]
    fn test_iterator() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_for_loop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        let mut results: Vec<i32> = Vec::new();
        for item in stack {
            results.push(item);
        }
        assert_eq!(results, vec![2, 1]);
    }

    #[test]
    fn test_iterator_with_strings() {
        let mut stack = Stack::new();
        stack.push(String::from("first"));
        stack.push(String::from("second"));
        let collected: Vec<String> = stack.into_iter().collect();
        assert_eq!(
            collected,
            vec![String::from("second"), String::from("first")]
        );
    }

    #[test]
    fn test_polymorphism() {
        let mut int_stack = Stack::new();
        int_stack.push(1);
        int_stack.push(2);
        assert_eq!(int_stack.pop(), Some(2));

        let mut string_stack = Stack::new();
        string_stack.push(String::from("hello"));
        string_stack.push(String::from("world"));
        assert_eq!(string_stack.pop(), Some(String::from("world")));

        let mut float_stack = Stack::new();
        float_stack.push(3.14_f64);
        float_stack.push(2.71_f64);
        assert_eq!(float_stack.pop(), Some(2.71_f64));
    }
}
