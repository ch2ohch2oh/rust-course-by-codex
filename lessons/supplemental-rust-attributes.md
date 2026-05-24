# Supplemental Lesson: Rust Attributes

## Goal

Understand what Rust attributes are, where they attach, and how common built-in and macro-driven attributes affect compilation, testing, layout, linting, and generated code.

## Why This Lesson Fits Here

Attributes are easiest to understand once you already know what items, modules, and crates are. They decorate those items and influence how the compiler or macros treat them. Placing this lesson after project structure and before traits, testing, async, and FFI gives you the right conceptual base before attributes start appearing everywhere else in the course.

## Mental Model

An attribute is metadata attached to an item or module. It can:

- enable or disable code conditionally
- derive trait implementations
- configure lint behavior
- influence memory layout
- feed input to procedural macros

For a C++ engineer, attributes are somewhere between standardized annotations, preprocessor-controlled compilation, and macro-triggered code generation. For a Python engineer, they are closer in spirit to decorators, but they work at compile time rather than as ordinary runtime functions.

## Syntax

The two main forms are:

- outer attribute: applies to the item that follows it
- inner attribute: applies to the enclosing item or module

Example:

```rust
#[derive(Debug, Clone)]
struct User {
    name: String,
}
```

Inner attribute example:

```rust
#![allow(dead_code)]
```

You will most often see inner attributes at the crate root and outer attributes on items.

## `derive`

`derive` asks a macro to generate trait implementations for you.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Config {
    retries: u32,
}
```

Common derives:

- `Debug`
- `Clone`
- `Copy`
- `PartialEq`
- `Eq`
- `Default`
- `Hash`

This is not "magic syntax" baked separately into the language. It is an attribute that invokes code generation.

### Design Note

Derive traits only when the generated semantics are actually correct for the type. For example, `Copy` is not a convenience marker you sprinkle everywhere. It changes ownership behavior.

## `cfg` and `cfg(test)`

`cfg` conditionally compiles code:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

This module exists only in test builds.

You can also target platform or feature configuration:

```rust
#[cfg(target_os = "macos")]
fn platform_name() -> &'static str {
    "macOS"
}
```

That is real conditional compilation, not a runtime `if`.

## `cfg_attr`

`cfg_attr` applies an attribute only when a condition is true:

```rust
#[cfg_attr(test, derive(Debug))]
struct ParsedToken {
    kind: u8,
}
```

This is useful when you want extra derives, documentation details, or lint behavior only in certain builds.

## Test-Related Attributes

You already see two common ones in the course:

- `#[cfg(test)]`
- `#[test]`

`#[test]` marks a function for the test harness:

```rust
#[test]
fn parses_value() {
    assert_eq!("42".parse::<i32>().unwrap(), 42);
}
```

Other test-related examples you will meet later in real projects include:

- `#[should_panic]`
- `#[ignore]`

## Lint-Control Attributes

Rust lets you adjust lint behavior with attributes:

```rust
#[allow(dead_code)]
fn helper_for_future_work() {}
```

At larger scope:

```rust
#![deny(unused_must_use)]
```

Common lint levels:

- `allow`
- `warn`
- `deny`
- `forbid`

Use these sparingly. A targeted `allow` with a good reason is fine. Blanket suppression is usually a smell.

## Layout Attributes: `repr`

`repr` controls layout guarantees:

```rust
#[repr(C)]
struct Header {
    tag: u32,
    len: u32,
}
```

Important forms:

- `#[repr(C)]`: C-compatible field ordering/layout rules
- `#[repr(transparent)]`: wrapper with the same representation as its single non-zero-sized field
- `#[repr(u8)]`, `#[repr(i32)]`: explicit enum representation

This matters for:

- FFI
- binary formats
- low-level systems work

Do not add `repr(C)` casually. Use it when you need a layout contract.

## Procedural Macro Attributes

Some attributes are provided by crates rather than the language:

```rust
#[tokio::main]
async fn main() {
    // ...
}
```

And:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Message {
    id: u64,
}
```

These are compile-time extensions. You do not need to understand procedural macro internals yet, but you should understand that attributes may come from libraries and can generate substantial code.

## Outer Versus Inner Attributes

Outer:

```rust
#[derive(Debug)]
struct Item;
```

Inner:

```rust
mod config {
    #![allow(dead_code)]
}
```

You usually reach for inner attributes when configuring an entire module or crate, not a single item.

## C++ and Python Notes

For C++ programmers:

- `cfg` is closer to structured conditional compilation than ad hoc preprocessor sprawl
- `derive` and proc-macro attributes are code generation, but more typed and compiler-integrated than textual macros
- `repr` is the place where you should think carefully about layout contracts

For Python programmers:

- attributes are not runtime decorators
- they act during compilation and can change what code exists at all

## Common Mistakes

- Treating `derive` as harmless boilerplate even when it changes semantics
- Using `#[allow(...)]` to silence warnings you should fix
- Assuming `cfg` branches are runtime branches
- Adding `repr(C)` without an actual ABI or layout requirement
- Forgetting that crate-provided attributes may expand into a lot of code

## Recap

Attributes are Rust's standard mechanism for attaching compile-time metadata to code. They matter because they shape what code exists, what traits are implemented, how strict the compiler is, and what layout contracts your types promise.

## Exercises

1. Add `#[derive(Debug, PartialEq)]` to a struct and verify the generated behavior.
2. Put tests behind `#[cfg(test)]`.
3. Use `#[allow(dead_code)]` in a small, targeted scope and explain why it is justified.
4. Add `#[repr(u8)]` to an enum and discuss when that kind of explicit representation matters.

## Exit Criteria

- You can explain the difference between `derive`, `cfg`, lint attributes, and `repr`.
- You know the difference between outer and inner attributes.
- You can recognize when an attribute changes semantics rather than only convenience.
