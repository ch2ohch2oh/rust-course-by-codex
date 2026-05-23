/*
Exercise 17: Testing and Documentation

Tasks:
1. Put reusable logic in `lib.rs` with doc comments.
2. Add at least two unit tests.
3. Add an integration test in `tests/`.
4. Run `cargo test`, `cargo fmt`, and `cargo clippy`.
*/

/// Returns true when the score is a passing grade.
///
/// # Examples
///
/// ```
/// assert!(testing_and_documentation::is_passing(75));
/// assert!(!testing_and_documentation::is_passing(40));
/// ```
pub fn is_passing(score: u32) -> bool {
    score >= 60
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passing_score_is_true() {
        assert!(is_passing(75));
    }

    #[test]
    fn failing_score_is_false() {
        assert!(!is_passing(40));
    }
}
