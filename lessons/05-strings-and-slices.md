# Part 5: Strings and Slices

## Goal

Understand the difference between owned strings and borrowed string slices, and learn why Rust string APIs are built around slices.

## Mental Model

Rust has two common string forms:

- `String`: owned, growable text
- `&str`: borrowed view into UTF-8 text

Most functions that only need to read text should take `&str`.

## `String` Versus `&str`

```rust
fn greet(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let owned = String::from("Rust");
    let borrowed = "world";

    greet(&owned);
    greet(borrowed);
}
```

This is flexible because both `String` and string literals can be used as `&str`.

If the function took `String`, callers would need to transfer ownership or allocate unnecessarily.

## Slices

A slice is a view into part of a collection. For strings:

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} {world}");
}
```

The slice borrows the original string. It does not allocate new text.

## A Useful Example: `first_word`

```rust
fn first_word(s: &str) -> &str {
    for (i, b) in s.as_bytes().iter().enumerate() {
        if *b == b' ' {
            return &s[..i];
        }
    }

    s
}
```

This demonstrates a classic Rust pattern:

- borrow text as `&str`
- return a slice tied to the same input
- avoid copying

Rust's standard library also gives you slice-oriented helpers such as `split`, `split_whitespace`, and `lines`. Those return iterators over borrowed pieces of the original string, which is often exactly what you want.

## Why String Indexing Is Restricted

This does not compile:

```rust
let s = String::from("hello");
// let c = s[0];
```

Rust forbids direct indexing because strings are UTF-8 and an integer index would not have one obvious meaning. Byte offsets, Unicode scalar values (`char`), and grapheme clusters are different concepts. A byte index might land in the middle of a code point, and even a code point index would still not match what users think of as a displayed character.

This is a correctness decision, not a missing feature.

## API Design Guidance

Prefer:

```rust
fn print_name(name: &str) { }
```

Over:

```rust
fn print_name(name: &String) { }
```

Why? `&str` accepts more callers and says more clearly that the function only needs a string view.

## C++ and Python Notes

For C++ programmers:

- `String` behaves more like an owning string object
- `&str` is closer to a non-owning validated string view

For Python programmers:

- Python strings are immutable, owned objects
- Rust separates ownership from borrowing much more explicitly

## Common Mistakes

- Using `&String` in APIs when `&str` is better
- Assuming a string index means character position
- Returning references to temporary strings

## Recap

String handling in Rust becomes much easier once you treat `String` as owned storage and `&str` as the standard read-only interface.

## Exercises

1. Implement `first_word` returning a slice.
2. Write a function that accepts `&str` and prints a greeting.
3. Split a comma-separated input line into borrowed parts using `split(',')`.
4. Compare using `String` and `&str` in function parameters.

## Exit Criteria

- You know when to use `String` and when to use `&str`.
- You can explain why direct indexing into a string is not allowed.
