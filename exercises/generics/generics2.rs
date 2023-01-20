// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        let test = Wrapper::new(42);
        assert_eq!(test.value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        let test = Wrapper::new("Foo");
        assert_eq!(test.value, "Foo");
    }
}
