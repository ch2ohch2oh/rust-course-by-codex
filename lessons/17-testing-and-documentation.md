# Part 17: Testing and Documentation

## Goal

Use Rust's built-in testing and documentation tools to make code safer to change and easier to understand.

## Mental Model

Rust treats tests and docs as part of the normal development workflow, not optional polish.

Core tools:

- `cargo test`
- `cargo doc`
- doctests from doc comments
- `cargo fmt`
- `cargo clippy`

## Unit Tests

Unit tests usually live in the same file as the code they test:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

This is good for small, focused behavior.

## Integration Tests

Integration tests live under `tests/` and exercise the public API from outside the crate.

This is useful when you want to validate behavior the way a real user of the crate would see it. In practice, that usually means putting reusable logic in `src/lib.rs` with `pub` functions or types, then keeping `main.rs` as a thin binary wrapper.

## Documentation Comments

Rust doc comments use `///`:

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let sum = mycrate::add(2, 3);
/// assert_eq!(sum, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Those examples can be run as doctests. That is a strong habit because it keeps documentation honest.

## Supporting Tools

- `cargo fmt`: enforce consistent formatting
- `cargo clippy`: catch suspicious or non-idiomatic code
- `cargo doc --open`: build API documentation

Treat these as normal development steps, not cleanup tasks at the end.

## What To Test

Prioritize:

- behavior with tricky edge cases
- public APIs that other code depends on
- logic that would be costly to debug later

Do not write tests that merely mirror the implementation line by line.

## Common Mistakes

- Testing only the happy path
- Letting examples in docs drift away from real code
- Leaving complex logic in `main` where it is harder to test

## Recap

Testing and documentation in Rust are tightly integrated with the toolchain. Lean on that integration rather than inventing separate workflows unless you need them.

## Exercises

1. Add unit tests to a utility module.
2. Add an integration test for a public API.
3. Write doc comments with a runnable example.
4. Run formatting and lint checks, then fix issues.

## Exit Criteria

- You can place tests in the right location.
- You can use documentation examples as executable checks.
