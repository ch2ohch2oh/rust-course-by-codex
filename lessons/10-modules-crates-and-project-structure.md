# Part 10: Modules, Crates, and Project Structure

## Goal

Learn how Rust organizes code into modules and crates so your projects stay maintainable as they grow.

## Mental Model

Rust organization happens at two levels:

- module: a namespace inside a crate
- crate: a single Rust compilation unit

Cargo adds another layer:

- package: the directory described by one `Cargo.toml`

A single Cargo package may contain multiple crates, commonly:

- one library crate from `src/lib.rs`
- one binary crate from `src/main.rs`
- additional binaries in `src/bin/`

## Modules

Suppose `src/lib.rs` contains:

```rust
mod math;

pub fn double(value: i32) -> i32 {
    math::multiply(value, 2)
}
```

And `src/math.rs` contains:

```rust
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

Here:

- `mod math;` declares the module
- `math.rs` defines it
- `pub` exposes items outside the module

## `use`

`use` brings names into scope:

```rust
use std::collections::HashMap;
```

It improves readability by shortening paths, but it does not copy code or change ownership.

## Binary Versus Library Crates

Use a binary crate when:

- the project is an app or CLI

Use a library crate when:

- you want reusable functionality
- you want to test core logic separately from the executable

Many real packages have both:

- `src/lib.rs` for core logic
- `src/main.rs` for command-line glue

## Privacy by Default

Rust defaults to private visibility. This is good design pressure.

Expose only what must be public. Keep helpers internal until you have a reason not to.

That means this is often better:

```rust
pub fn parse_config(text: &str) -> Config { /* ... */ }
```

Than exposing every helper function involved in parsing.

## Suggested Structure

For a medium project:

- `src/main.rs`
- `src/lib.rs`
- `src/config.rs`
- `src/parser.rs`
- `src/model.rs`
- `tests/`

Let the binary be thin. Put most logic in the library so it is easier to test.

## C++ and Python Notes

For C++ programmers:

- modules and crates replace a large part of the header/source organization burden
- there is far less tolerance for vaguely exposed global surfaces

For Python programmers:

- project structure is more explicit and compile-checked
- imports and visibility are stricter and clearer

## Common Mistakes

- Making everything `pub`
- Leaving all logic in `main.rs`
- Splitting files too early without a coherent API boundary

## Recap

Good Rust structure starts with small private modules and grows outward only when reuse or public API needs justify it.

## Exercises

1. Split a single-file project into at least three modules.
2. Create a library crate with a public utility API.
3. Add a binary target that calls into the library crate from the same package.
4. Reduce unnecessary `pub` visibility.

## Exit Criteria

- You can explain what belongs in `src/main.rs`, `src/lib.rs`, and module files.
- You can keep internal details private by default.
