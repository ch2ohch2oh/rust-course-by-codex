# Part 2: Variables, Mutability, and Basic Types

## Goal

Understand how Rust binds values to names, why values are immutable by default, and how the basic built-in types work.

## Mental Model

Rust makes state changes explicit. A binding created with `let` cannot be changed unless you opt in with `mut`.

That sounds minor, but it shapes the style of the language:

- immutable-by-default code is easier to reason about
- accidental state changes are less common
- the compiler can make stronger guarantees

## Variables and Mutability

```rust
fn main() {
    let x = 5;
    let mut y = 10;

    y = 12;

    println!("x = {x}, y = {y}");
}
```

This will compile because only `y` is mutable.

This will not compile:

```rust
fn main() {
    let x = 5;
    x = 6;
}
```

Rust rejects it because `x` is immutable.

## Shadowing

Shadowing creates a new binding with the same name:

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}");
}
```

This is different from mutation:

- mutation changes the same variable
- shadowing creates a new variable that replaces the old name in scope

Shadowing is useful when transforming data from one type into another.

## Scalar Types

Main scalar types:

- integers: `i32`, `u64`, and others
- floating point: `f32`, `f64`
- boolean: `bool`
- character: `char`

Example:

```rust
fn main() {
    let age: u32 = 30;
    let temperature: f64 = 21.5;
    let is_rust_fun: bool = true;
    let initial: char = 'R';

    println!("{age} {temperature} {is_rust_fun} {initial}");
}
```

## Compound Types

### Tuples

```rust
fn main() {
    let person = ("Ava", 29, true);
    let (name, age, active) = person;

    println!("{name} {age} {active}");
}
```

### Arrays

Arrays have fixed length and fixed element type:

```rust
fn main() {
    let scores = [10, 20, 30, 40, 50];
    println!("{}", scores[0]);
}
```

Use arrays for fixed-size data. Use `Vec<T>` later for growable collections.

## No Implicit Numeric Conversions

Rust avoids silent conversions:

```rust
fn main() {
    let x: i32 = 5;
    let y: i64 = 10;

    let sum = i64::from(x) + y;
    println!("{sum}");
}
```

This is stricter than both Python and much of everyday C++ code, but it prevents subtle bugs.

## C++ and Python Notes

For C++ programmers:

- Rust's default immutability is stricter than common C++ style
- explicit conversions are more common and more intentional

For Python programmers:

- types are fixed at compile time
- array element types must match
- changing a name from a string to an integer uses shadowing, not loose runtime rebinding

## Common Mistakes

- Using `mut` everywhere instead of designing for immutability
- Confusing shadowing with mutation
- Expecting `i32 + i64` to compile
- Expecting arrays to resize dynamically

## Recap

This part gives you the baseline vocabulary of Rust values. The next important shift is ownership: who owns those values and when that ownership moves.

## Exercises

1. Build a temperature converter for Celsius and Fahrenheit.
2. Use a tuple to store a person's name and age.
3. Use an array to store five numbers and print their sum.
4. Show the difference between mutation and shadowing in one program.

## Exit Criteria

- You can choose between `mut` and shadowing intentionally.
- You can explain why Rust avoids implicit numeric conversion.
