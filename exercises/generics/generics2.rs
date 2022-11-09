// This powerful wrapper provides the ability to store a positive integer value.
// 使用仿制药对其进行重写，以便它支持包装任何类型.

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
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
