# Part 12: Lifetimes

## Goal

Understand lifetime annotations as a way to describe relationships between references, not as manual memory management.

## Mental Model

Lifetimes tell Rust how borrowed references relate to one another. They do not change how long data lives. They only describe constraints the compiler must enforce.

The most important rule is this:

- a reference must never outlive the data it points to

## Why Lifetime Annotations Exist

Consider:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The annotation says:

- `x` and `y` are both valid for at least lifetime `'a`
- the returned reference is valid for that same lifetime

In practice, the returned reference is valid only as long as both inputs are valid. Rust uses `'a` to express that relationship.

## Lifetime Elision

Rust often infers lifetimes for you:

```rust
fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}
```

You do not need explicit annotations here because the relationship is obvious under the language's elision rules.

Use explicit annotations when:

- a function has multiple input references
- the return value depends on them
- a struct stores references

## Structs with References

```rust
struct Excerpt<'a> {
    text: &'a str,
}
```

This means an `Excerpt` cannot outlive the string slice it contains.

## A Common Misunderstanding

Beginners often think:

"If I add more lifetime annotations, the program will live longer."

That is false. Lifetimes do not extend data lifetimes. They only describe what is already valid.

## When Ownership Is Better

If lifetime annotations become painful, ask whether the type should own the data instead:

```rust
struct OwnedExcerpt {
    text: String,
}
```

Owning data often simplifies APIs at the cost of allocation or copying.

This is a good trade when:

- the referenced data will not naturally outlive the struct
- API complexity is becoming high
- you need independence from the original source buffer

## C++ and Python Notes

For C++ programmers:

- lifetimes are Rust's way of statically preventing dangling references
- you do not manually manage them as you would reason about raw pointer validity

For Python programmers:

- this is one of the biggest conceptual jumps because Python hides memory lifetime management almost completely

## Common Mistakes

- Treating lifetimes as values rather than relationships
- Returning references to local variables
- Forcing borrowed designs where owned data would be simpler

## Recap

Lifetimes are difficult mainly until you stop treating them as a syntax puzzle. They exist to describe borrowing relationships precisely enough for the compiler to reject invalid references.

## Exercises

1. Implement the classic `longest` function.
2. Define a struct that stores a string slice reference.
3. Fix a lifetime error either by annotating or by changing ownership.
4. Explain in comments what relationship each lifetime annotation expresses.

## Exit Criteria

- You understand that lifetimes describe relationships, not durations you manually manage.
- You can decide when to stop fighting lifetimes and own the data instead.
