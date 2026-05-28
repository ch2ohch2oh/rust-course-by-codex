# Part 13: Smart Pointers and Heap Data

## Goal

Learn when plain ownership is enough and when smart pointers are needed to express more complex memory relationships.

## Mental Model

Most Rust code uses ordinary ownership and borrowing. Smart pointers are for specific cases:

- `Box<T>`: single ownership of heap data
- `Rc<T>` (`Reference Counted<T>`): shared ownership in single-threaded code
- `Arc<T>` (`Atomically Reference Counted<T>`): shared ownership in multi-threaded code
- `RefCell<T>`: interior mutability with runtime borrow checks

Do not reach for them automatically. Start with normal ownership first.

## `Box<T>`

`Box<T>` stores a value on the heap and owns it uniquely.

Useful for:

- recursive types
- putting a value behind a stable level of indirection when the type layout requires it

Example:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Without `Box`, the enum would have infinite size at compile time.

## `Rc<T>`

`Rc<T>` stands for `Reference Counted<T>`. It enables multiple owners in single-threaded code:

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("shared"));
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    println!("{a} {b}");
}
```

`Rc` uses reference counting. The data is dropped when the last owner goes away.

## `RefCell<T>`

`RefCell<T>` lets you mutate through an immutable outer value by enforcing borrowing rules at runtime:

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(5);
    *value.borrow_mut() += 1;
    println!("{}", value.borrow());
}
```

This is useful when the compiler cannot easily prove the borrow rules, but your design still guarantees correctness.

If you break the borrow rules, `RefCell` panics at runtime.

## `Rc<RefCell<T>>`

This combination is common:

- `Rc` for shared ownership
- `RefCell` for mutation behind shared references

It is useful, but it is also a warning sign. If you use it everywhere, your design may be drifting back toward loosely controlled shared mutable state.

## `Arc<T>`

`Arc<T>` stands for `Atomically Reference Counted<T>`. It is the thread-safe version of `Rc<T>`.

Use it when shared ownership crosses threads. It is often combined with `Mutex<T>` for shared mutable state.

## `Weak<T>` and Cycles

Reference counting does not break cycles automatically. If two `Rc<T>` values point at each other, they will leak.

That is why tree and graph structures often need `Weak<T>` for back-references such as:

- child to parent
- graph node to observer

Use strong references for ownership and weak references for non-owning links.

## Design Guidance

Choose the simplest tool:

- plain ownership if possible
- `Box` if you just need indirection
- `Rc` if you need shared ownership in one thread
- `Arc` if ownership is shared across threads
- `RefCell` only when compile-time borrowing is too restrictive for a justified design

## Common Mistakes

- Using `Rc<RefCell<T>>` as a default architecture
- Reaching for shared ownership before trying clearer ownership models
- Forgetting that `Rc<T>` is not thread-safe

## Recap

Smart pointers are for expressing non-trivial ownership relationships. They are powerful precisely because they are not the default.

## Exercises

1. Build a recursive list type using `Box`.
2. Share read-only data with `Rc`.
3. Create a small graph- or tree-like structure and discuss where `Weak<T>` would be needed to avoid cycles.
4. Use `RefCell` to mutate through a shared owner and observe runtime rules.

## Exit Criteria

- You can explain why `Rc<RefCell<T>>` is powerful but easy to overuse.
- You can pick the simplest smart pointer that solves the problem.
