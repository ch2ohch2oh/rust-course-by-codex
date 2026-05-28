# Part 14: Closures and Functional Style

## Goal

Use closures comfortably and write iterator-heavy code that stays readable under Rust's ownership rules.

## Mental Model

A closure is an anonymous function that can capture values from its environment.

Rust closures may capture by:

- shared borrow
- mutable borrow
- move

The compiler infers the least powerful capture mode that works, unless you force a move.

At the trait level, closures implement one or more of:

- `Fn`: can be called through a shared borrow
- `FnMut`: needs mutable access to captured state
- `FnOnce`: consumes captured values when called

Those traits matter because many APIs are expressed in terms of them.

Quick cheat sheet:

| Trait | What it does | Typical capture behavior |
| --- | --- | --- |
| `Fn` | read captured values | shared borrow |
| `FnMut` | mutate captured values | mutable borrow |
| `FnOnce` | consume captured values | move |

Examples:

```rust
let x = 5;
let show = || println!("{x}"); // `Fn`

let mut count = 0;
let mut bump = || count += 1; // `FnMut`

let name = String::from("Rust");
let consume = move || drop(name); // `FnOnce`
```

## Closure Syntax

```rust
let add_one = |x: i32| x + 1;
println!("{}", add_one(5));
```

Closures are heavily used with iterators, sorting, and callbacks.

## Capturing Environment

```rust
fn main() {
    let factor = 3;
    let multiply = |x| x * factor;

    println!("{}", multiply(4));
}
```

Here the closure borrows `factor`.

Mutable capture:

```rust
fn main() {
    let mut count = 0;
    let mut increment = || {
        count += 1;
    };

    increment();
    increment();
    println!("{count}");
}
```

## `move` Closures

Sometimes a closure must take ownership:

```rust
use std::thread;

fn main() {
    let text = String::from("hello");
    let handle = thread::spawn(move || {
        println!("{text}");
    });

    handle.join().unwrap();
}
```

`move` is especially common when spawning threads or async tasks, because the closure may run after the current scope would otherwise end.

## Closures with Iterators

```rust
fn main() {
    let words = vec!["1", "2", "bad", "3"];

    let numbers: Vec<i32> = words
        .iter()
        .filter_map(|w| w.parse::<i32>().ok())
        .collect();

    println!("{numbers:?}");
}
```

This is an idiomatic pattern:

- transform each item
- drop invalid items
- collect results

## Sorting with Closures

```rust
#[derive(Debug)]
struct User {
    name: String,
    score: u32,
}

fn main() {
    let mut users = vec![
        User { name: "A".into(), score: 30 },
        User { name: "B".into(), score: 10 },
    ];

    users.sort_by_key(|u| u.score);
    println!("{users:?}");
}
```

## Readability Rule

Prefer iterator chains when each stage is easy to name mentally. If the code needs several comments to explain a closure pipeline, a loop may be the better choice.

## Common Mistakes

- Forgetting a closure may capture and therefore affect ownership
- Using `move` without understanding that it transfers values into the closure
- Writing dense iterator code that is technically elegant but hard to maintain

## Recap

Closures are where ownership and functional style meet. Once capture behavior becomes intuitive, a lot of Rust APIs feel much more natural.

## Exercises

1. Sort a list of structs with `sort_by_key`.
2. Write a closure that captures environment state.
3. Parse a list of strings into numbers using iterator adapters.
4. Refactor a multi-step loop into a readable iterator pipeline.

## Exit Criteria

- You can predict how a closure captures values.
- You can keep functional-style code readable under ownership rules.
