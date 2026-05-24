# Part 8: Collections and Iterators

## Goal

Use Rust's standard collections and iterator system to write clear data-processing code.

## Mental Model

The two most important early collections are:

- `Vec<T>`: growable contiguous sequence
- `HashMap<K, V>`: key-value lookup table

Rust iteration is lazy by default. You build iterator pipelines and only execute them when a consumer such as `collect`, `sum`, or `for_each` runs.

## `Vec<T>`

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);

    for n in &numbers {
        println!("{n}");
    }
}
```

Use `Vec<T>` when the number of items may change.

## `HashMap<K, V>`

```rust
use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    counts.insert("rust", 1);
    counts.insert("python", 2);

    println!("{:?}", counts.get("rust"));
}
```

Map lookups return `Option<&V>` because the key may be missing.

## Iterator Basics

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers
        .iter()               // Iter<'_, i32>
        .map(|n| n * 2)       // Map<Iter<'_, i32>, _>
        .filter(|n| *n > 5)   // Filter<Map<Iter<'_, i32>, _>, _>
        .collect();           // Vec<i32>

    println!("{doubled:?}");
}
```

This pipeline:

- borrows each item with `iter()`
- transforms with `map`
- filters with `filter`
- materializes a new vector with `collect`

## `iter`, `iter_mut`, and `into_iter`

- `iter()`: shared references, item type is usually `&T`
- `iter_mut()`: mutable references, item type is usually `&mut T`
- `into_iter()`: consumes the collection and yields owned items

Example:

```rust
let v = vec![1, 2, 3];

for x in v.iter() {
    println!("{x}");
}
```

After `iter()`, `v` is still available. After `into_iter()`, it is consumed.

## When To Use Loops Versus Iterators

Prefer iterators when:

- the transformation pipeline is short and direct
- ownership behavior is obvious
- the code becomes more declarative

Prefer explicit loops when:

- stateful logic becomes awkward in a chain
- readability is better with named steps

Idiomatic Rust does not mean "always use the cleverest iterator chain."

## Word Count Example

```rust
use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        // `entry(word)` returns `Entry<&str, usize>`, and `or_insert(0)` gives `&mut usize`.
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}
```

This introduces the `entry` API, which is a very common `HashMap` pattern.

The returned keys borrow from `text`, so the map cannot outlive the input string. That can be efficient, but it also means `HashMap<&str, usize>` is appropriate only when the source text stays alive long enough. If the map must own its keys independently, use `HashMap<String, usize>` instead.

## Common Mistakes

- Consuming a collection with `into_iter()` when you still need it
- Collecting intermediate vectors unnecessarily
- Forcing iterator chains when a simple loop is clearer

## Recap

Collections hold your data; iterators are how you transform it idiomatically. Understanding the ownership implications of each iterator flavor is the important part.

## Exercises

1. Compute average, median, and mode for a list of numbers.
2. Count word frequencies with `HashMap`.
3. Filter even numbers from a vector and collect them into a new vector.
4. Rewrite a manual loop as an iterator chain.

## Exit Criteria

- You can decide between loops and iterators pragmatically.
- You understand the difference between `iter`, `iter_mut`, and `into_iter`.
