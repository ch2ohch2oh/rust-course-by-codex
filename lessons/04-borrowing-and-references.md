# Part 4: Borrowing and References

## Goal

Use references to access data without taking ownership, and understand the rules Rust applies to keep that safe.

## Mental Model

Borrowing means: "I want temporary access to this value, but I do not want to own it."

Rust supports two main forms:

- shared reference: `&T`
- mutable reference: `&mut T`

The key safety rule is this:

- you may have many readers
- or one writer
- but not both at the same time

## Shared References

```rust
fn length(text: &str) -> usize {
    text.len()
}

fn main() {
    let s = String::from("hello");
    let n = length(&s);

    println!("{s} has length {n}");
}
```

`length` borrows `s`, so `main` still owns it afterward. `&str` is the better default here because it accepts both `String` and string literals.

## Mutable References

```rust
fn add_suffix(text: &mut String) {
    text.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    add_suffix(&mut s);
    println!("{s}");
}
```

To mutate through a reference:

- the original binding must be mutable
- the function must accept `&mut T`

## The Borrowing Rule

This fails:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;

    println!("{r1} {r2}");
}
```

Rust rejects it because immutable and mutable borrows overlap.

This version is fine:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{r1}");

    let r2 = &mut s;
    r2.push('!');
    println!("{r2}");
}
```

The immutable borrow ends before the mutable borrow begins.

## Why Rust Is Strict Here

If you allow mutation while other aliases are reading, you invite:

- stale reads
- invalidated references
- thread-safety problems later

Rust pushes these errors to compile time.

## API Design Note

Borrowing often gives better APIs than ownership transfer. If a function only needs to inspect data, prefer a shared reference. If it only needs to modify temporary caller-owned data, prefer a mutable reference.

## C++ and Python Notes

For C++ programmers:

- references exist in both languages, but Rust enforces aliasing rules much more aggressively
- many bugs that are possible with raw pointers or references in C++ are rejected here

For Python programmers:

- passing an object to a function never produces a borrow checker error in Python
- Rust forces you to model read-only versus mutable access explicitly

## Common Mistakes

- Taking ownership when borrowing would do
- Using `&String` where `&str` would be more flexible
- Trying to hold a mutable borrow across too much code

## Recap

Ownership tells you who is responsible for cleanup. Borrowing tells you how other code may temporarily access that value safely.

## Exercises

1. Write a function that takes `&str` and returns its length.
2. Write a function that appends text using `&mut String`.
3. Intentionally create a conflicting borrow and then fix it.
4. Rewrite one ownership-heavy example from Part 3 using borrowing.

## Exit Criteria

- You can pass large values by reference.
- You can explain why Rust rejects simultaneous mutable and immutable access.
