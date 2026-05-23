mod arithmetic;

pub use arithmetic::multiply;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_returns_expected_result() {
        assert_eq!(multiply(6, 7), 42);
    }
}
