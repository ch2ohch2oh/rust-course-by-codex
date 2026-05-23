# Part 9: Error Handling

## Goal

Handle missing values and operational failures explicitly with `Option` and `Result`.

## Mental Model

Rust separates two common failure categories:

- `Option<T>`: a value may or may not be present
- `Result<T, E>`: an operation may succeed or fail

This pushes error handling into the type system instead of hiding it in runtime conventions.

## `Option<T>`

```rust
fn first(values: &[i32]) -> Option<i32> {
    values.first().copied()
}
```

If the slice is empty, there is no first value. That is not necessarily an "error"; it is simply absence.

Use `match`:

```rust
match first(&[10, 20, 30]) {
    Some(value) => println!("{value}"),
    None => println!("no value"),
}
```

## `Result<T, E>`

```rust
fn parse_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    text.parse::<i32>()
}
```

This makes failure explicit in the return type.

## The `?` Operator

`?` propagates errors upward:

```rust
use std::fs;
use std::io;

fn read_file(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
```

If reading fails, the function returns early with the error.

For `Result`, `?` also performs error conversion when needed through `From`. That is why a function returning a broader error type can often use `?` on narrower error values without manual mapping every time.

## `main` Can Return `Result`

```rust
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let text = fs::read_to_string("notes.txt")?;
    println!("{text}");
    Ok(())
}
```

This is often cleaner than calling `unwrap`.

## `unwrap` and `expect`

These are allowed, but use them carefully.

- `unwrap()`: panic on error
- `expect("message")`: panic with your message

They are acceptable in:

- tiny throwaway programs
- tests
- places where failure is truly unrecoverable and well-justified

They are not good default error handling for real code.

## C++ and Python Notes

For C++ programmers:

- this is more explicit than exceptions and more structured than mixed return-code conventions
- the compiler makes error propagation visible in signatures

For Python programmers:

- failures are not silently hidden in control flow
- many operations that would raise at runtime in Python require explicit handling here

## Common Mistakes

- Using `Option` where you need an actual error message
- Using `unwrap` everywhere because it feels faster
- Defining vague error APIs that discard useful information

## Recap

Rust error handling is explicit by design. Once you accept that a function signature should reveal failure modes, the rest becomes much more coherent.

## Exercises

1. Parse integers from input and report invalid values.
2. Open a file and print either its contents or a friendly error.
3. Refactor a program that uses `unwrap` everywhere into one returning `Result`.
4. Chain multiple fallible operations with `?`.

## Exit Criteria

- You can choose between `Option` and `Result`.
- You can write a `main` function that returns `Result`.
