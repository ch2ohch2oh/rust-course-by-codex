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

Simple types often implement `Copy`:

```rust
fn main() {
    let x = 5;
    let y = x;

    println!("{x} {y}");
}
```

Here `x` remains usable because integers implement `Copy`.

Typical `Copy` types include:

- integers
- booleans
- chars
- tuples containing only `Copy` types

User-defined structs are also non-`Copy` by default unless all their fields are `Copy` and you opt into `Copy`.

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
