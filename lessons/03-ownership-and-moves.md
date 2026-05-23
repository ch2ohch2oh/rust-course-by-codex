# Part 3: Ownership and Moves

## Goal

Learn the ownership model that Rust uses to manage memory safely without a garbage collector.

## Mental Model

Every value in Rust has an owner. When the owner goes out of scope, the value is dropped. Some assignments and function calls transfer ownership instead of copying the value.

This is the core rule set:

- each value has one owner
- ownership can move
- when the owner goes out of scope, cleanup runs automatically

## A Scope Example

```rust
fn main() {
    let name = String::from("Rust");
    println!("{name}");
}
```

When `main` ends, `name` goes out of scope and its memory is released.

## Moves

Assignment of a non-`Copy` value moves ownership by default. `String` is a common first example:

```rust
fn main() {
    let a = String::from("hello");
    let b = a;

    println!("{b}");
}
```

After `let b = a;`, `a` is no longer valid.

Why? Because `a` and `b` would otherwise both act like owners of the same resource. Rust solves that by invalidating the old binding after the move.

The rule is not "heap values move, stack values copy." The real distinction is:

- `Copy` types are duplicated implicitly
- non-`Copy` types move by default

Many non-`Copy` types do own heap data, but that is an example, not the definition.

## Clone

If you want a deep copy, say so explicitly:

```rust
fn main() {
    let a = String::from("hello");
    let b = a.clone();

    println!("{a}");
    println!("{b}");
}
```

`clone` is explicit because copying heap data may be expensive.

## `Copy` Types

`Copy` is a special **marker trait** in Rust that changes the default assignment behavior from **moving** to **bitwise copying**.

If a type implements the `Copy` trait, assigning it to a new variable creates a fast, bit-by-bit copy of the data. Because the data is simply duplicated, **both the old and the new variables remain perfectly valid and usable**.

Types that live entirely on the stack and don't manage extra resources (like heap memory or file handles) are usually `Copy`:

```rust
fn main() {
    let x = 5;
    let y = x; // A bitwise copy happens here

    // This works fine! Both x and y are valid.
    println!("x is {}, y is {}", x, y);
}
```

Typical `Copy` types include:

- Integers (`i32`, `u8`, etc.)
- Floats (`f64`, `f32`)
- Booleans (`bool`)
- Characters (`char`)
- Tuples containing only `Copy` types

### `Copy` and Structs

When you create your own user-defined `struct`, Rust makes it **non-`Copy` (it moves)** by default to be safe. 

If you want your struct to copy instead of move, you must explicitly opt in by adding `#[derive(Copy, Clone)]` above it. The compiler will only allow this if *every single field* inside your struct is also a `Copy` type.

```rust
// This struct can be Copy because i32 and f64 are Copy
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: f64,
}

// This struct CANNOT be Copy because String is not Copy
struct User {
    name: String, // String manages heap memory, so it must move
}
```

## Ownership and Functions

Passing an owned value to a function usually moves it:

```rust
fn print_text(text: String) {
    println!("{text}");
}

fn main() {
    let s = String::from("hi");
    print_text(s);
}
```

After the call, `s` is no longer valid in `main`.

This often surprises Python programmers because passing an object reference in Python does not invalidate the caller binding.

For C++ programmers, the important distinction is that Rust assignment of a non-`Copy` value is a move by default, and the source binding becomes unusable afterward. That is different from C++'s user-defined move constructor and move-assignment model. Rust moves are usually best understood as ownership transfer plus source invalidation.

## Why This Exists

Ownership lets Rust prevent:

- use-after-free
- double free
- many lifetime bugs

The price is that you must be more explicit about who controls each value.

## Common Mistakes

- Cloning by reflex instead of fixing the ownership design
- Thinking assignment means copy for all types
- Fighting moves instead of passing references when ownership transfer is unnecessary

## Recap

If a type is not `Copy`, assume assignment and function calls may move it. In the next part, you will learn how borrowing lets you use data without taking ownership.

## Exercises

1. Pass a `String` into a function and observe that the caller can no longer use it.
2. Repeat with `clone` and compare behavior.
3. Repeat with an `i32` and explain why the caller still can use it.
4. Write comments describing who owns the value after each line.

## Exit Criteria

- You can predict when a value moves.
- You can explain why `String` and `i32` behave differently.
