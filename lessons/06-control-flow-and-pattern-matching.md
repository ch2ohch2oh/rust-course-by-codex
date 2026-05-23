# Part 6: Control Flow and Pattern Matching

## Goal

Learn Rust's control-flow tools and use pattern matching as a core part of everyday design.

## Mental Model

Rust has familiar loops and conditionals, but `match` is more central than in C++ or Python. Once you start modeling states with enums, pattern matching becomes one of the language's main strengths.

## Basic Control Flow

```rust
fn main() {
    let number = 7;

    if number % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
}
```

Rust `if` requires a boolean expression. There is no truthy/falsy conversion like Python.

## Loops

### Infinite Loop

```rust
loop {
    break;
}
```

### While Loop

```rust
let mut n = 3;
while n > 0 {
    println!("{n}");
    n -= 1;
}
```

### For Loop

```rust
for x in [10, 20, 30] {
    println!("{x}");
}
```

`for` over iterables is usually preferred to manual indexing.

## `match`

```rust
enum Command {
    Start,
    Stop,
    Pause,
}

fn handle(cmd: Command) {
    match cmd {
        Command::Start => println!("starting"),
        Command::Stop => println!("stopping"),
        Command::Pause => println!("pausing"),
    }
}
```

`match` must be exhaustive. That means you must handle every case. This is extremely valuable when your enum grows and the compiler points out all code that must be updated.

`match` is also an expression, not just a branching statement:

```rust
let label = match number {
    0 => "zero",
    1 => "one",
    _ => "many",
};
```

## `if let`

If you only care about one pattern, `if let` is shorter:

```rust
let maybe_name = Some(String::from("Ava"));

if let Some(name) = maybe_name {
    println!("{name}");
}
```

Use `if let` when:

- one case matters
- the others can be ignored

Use `match` when:

- you need all cases
- you want exhaustiveness

## `while let`

Useful when repeatedly extracting values:

```rust
let mut stack = vec![1, 2, 3];

while let Some(value) = stack.pop() {
    println!("{value}");
}
```

## C++ and Python Notes

For C++ programmers:

- `match` gives stronger exhaustiveness guarantees than a traditional `switch`
- enum variants can carry data in a much richer way

For Python programmers:

- `match` in Rust is not just syntactic sugar; it is deeply tied to type-safe enum design
- `if` does not accept arbitrary truthy values

## Common Mistakes

- Overusing `if/else` where `match` is clearer
- Using indexed loops instead of iterator-based loops
- Ignoring how exhaustive matching reduces bugs during refactors

## Recap

This part is where Rust starts to feel distinct. Pattern matching is not an advanced trick; it is one of the normal tools for writing clear code.

## Exercises

1. Build a small enum-driven menu or state machine and branch on it with `match`.
2. Define an enum with at least three variants and match on it.
3. Convert an `if/else if/else` chain into `match`.
4. Use `while let` to consume values from a vector or stack.

## Exit Criteria

- You can use `match` confidently for branching on enums.
- You can explain when `if let` is cleaner than full `match`.
