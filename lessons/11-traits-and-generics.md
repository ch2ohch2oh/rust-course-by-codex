# Part 11: Traits and Generics

## Goal

Write reusable code without sacrificing type safety, and understand how Rust expresses shared behavior without inheritance.

## Mental Model

Generics let you abstract over types. Traits let you abstract over behavior.

Together they cover a large part of what templates and interfaces or protocols do in other languages, but in a way that is explicit and strongly checked.

## Generic Functions

```rust
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut largest = items.first()?;

    for item in &items[1..] {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}
```

This works for any type `T` that:

- can be compared with `PartialOrd`

It also handles empty input explicitly instead of assuming the slice is non-empty.
## Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

Usage:

```rust
let p = Point { x: 3, y: 4 };
let q = Point { x: 1.5, y: 2.5 };
```

Both fields must use the same `T` here. You can also define `Point<T, U>` if you need different types.

## Traits

```rust
trait Describe {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("Book: {}", self.title)
    }
}
```

Traits define required behavior. Types implement that behavior explicitly.

If you come from Java, TypeScript, or C#, a trait is the closest Rust equivalent to an interface. That said, "shared behavior contract" is a better mental model because traits can also include default method implementations and are commonly used in generic bounds.

## Trait Bounds

```rust
fn print_description<T: Describe>(item: &T) {
    println!("{}", item.describe());
}
```

This says: accept any type implementing `Describe`.

You may also see:

```rust
fn print_description(item: &impl Describe) {
    println!("{}", item.describe());
}
```

This is often shorter and easier to read.

## `where` Clauses

Long bounds become clearer with `where`:

```rust
fn compare_and_print<T>(a: &T, b: &T)
where
    T: Describe + PartialOrd,
{
    if a < b {
        println!("first is smaller: {}", a.describe());
    } else {
        println!("first is not smaller: {}", a.describe());
    }
}
```

For C++ programmers, a `where` clause is similar to a `requires` clause using concepts: it expresses the capabilities a generic type must have, but Rust usually does that by naming traits in the bound.

## Design Guidance

Use generics when:

- the behavior is truly identical across types
- the abstraction improves the API

Do not genericize everything just because you can. Concrete code is easier to read when reuse is not real.

## C++ and Python Notes

For C++ programmers:

- traits are not class inheritance
- generic bounds are usually easier to reason about than unconstrained template metaprogramming

For Python programmers:

- duck typing becomes explicit compile-time contracts
- shared behavior is described before use, not assumed at runtime

## Common Mistakes

- Writing overly abstract code too early
- Using traits where a simple enum would model a closed set of cases better
- Adding bounds you do not actually need

## Recap

Traits and generics are how Rust scales code reuse. The important habit is to keep abstractions honest: use them when they clarify the design, not when they make it look advanced.

## Exercises

1. Write a generic `max` function.
2. Define a `Describe` trait and implement it for multiple types.
3. Create a generic struct such as `Point<T>`.
4. Write a function that accepts any type implementing a trait bound.

## Exit Criteria

- You can express shared behavior with traits instead of inheritance.
- You can read a function signature with multiple trait bounds.
