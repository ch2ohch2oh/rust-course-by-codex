# Part 7: Structs, Enums, and Methods

## Goal

Model data clearly with structs and enums, then attach behavior in idiomatic Rust style.

## Mental Model

Rust pushes you toward explicit data modeling. Instead of relying on loose dictionaries, parallel variables, or inheritance-heavy object hierarchies, you describe your state directly.

Use:

- `struct` for "this thing has fields"
- `enum` for "this thing can be one of several variants"
- `impl` for associated behavior

## Structs

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

Create and use a value:

```rust
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{} {}", rect.width, rect.height);
}
```

## Methods

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

`&self` means the method borrows the instance. It does not take ownership.

## Associated Functions

Functions inside `impl` blocks that do not take `self` are associated functions:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

Called like:

```rust
let sq = Rectangle::square(10);
```

## Enums

Enums are stronger than simple tagged constants because variants may carry data:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

This lets the type system encode valid states directly.

## Pattern Matching on Enums

```rust
fn process(msg: Message) {
    match msg {
        Message::Quit => println!("quitting"),
        Message::Move { x, y } => println!("move to {x}, {y}"),
        Message::Write(text) => println!("text: {text}"),
    }
}
```

This is one of the most important Rust design patterns: enums plus `match`.

## C++ and Python Notes

For C++ programmers:

- enums replace many tagged-base-class or flag-heavy designs; traits later replace many interface-style uses of inheritance
- algebraic data modeling is more central than class hierarchies

For Python programmers:

- structs and enums provide much stronger guarantees than ad hoc dict-based state
- you get compile-time checking that all variants are handled

## Common Mistakes

- Using multiple booleans where an enum would model state more honestly
- Turning every helper into a method even when a free function is clearer
- Taking ownership in methods when borrowing is enough

## Recap

This part gives you the basic language for modeling program state. Rust becomes much easier once your types reflect reality instead of vaguely shaped data.

## Exercises

1. Create a `Rectangle` struct with `width` and `height`.
2. Add an `area` method and a `can_hold` method.
3. Create a `Message` enum with multiple variants, including one carrying data.
4. Write a method or function that reacts to each `Message` variant.

## Exit Criteria

- You can model state with enums instead of ad hoc flags.
- You can distinguish methods from free functions.
