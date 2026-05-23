# Part 18: Unsafe Rust and FFI

## Goal

Understand what `unsafe` actually means, why it exists, and how to isolate it behind small, defensible interfaces.

## Mental Model

`unsafe` does not turn off Rust completely. It marks code where the compiler cannot verify some safety guarantees, so you are responsible for upholding them.

Common reasons to use it:

- interacting with raw pointers
- calling foreign code
- implementing low-level data structures
- accessing certain performance-sensitive primitives

## What `unsafe` Allows

Unsafe Rust permits operations such as:

- dereferencing raw pointers
- calling `unsafe fn`
- accessing mutable statics
- implementing unsafe traits

Example:

```rust
fn main() {
    let x = 10;
    let ptr = &x as *const i32;

    unsafe {
        println!("{}", *ptr);
    }
}
```

This compiles only because the dereference is inside `unsafe`.

## Safety Contracts

Whenever you write unsafe code, ask:

- what assumptions must hold?
- who is responsible for maintaining them?
- can I reduce the unsafe surface?

A good pattern is:

- keep unsafe code in a tiny internal block
- expose a safe wrapper only when the wrapper can actually enforce the invariant
- document the invariants

## Safe Wrapper Pattern

Instead of spreading raw pointer logic across the codebase, isolate it:

```rust
unsafe fn read_ptr(ptr: *const i32) -> i32 {
    *ptr
}
```

This keeps the contract honest: the caller must guarantee that `ptr` is valid, aligned, non-null, and points to a live `i32`. A safe wrapper is appropriate only when the wrapper itself can establish those facts, for example by taking `&i32` instead of `*const i32`.

## FFI

Rust interoperates with C through a stable C ABI.

Typical FFI work includes:

- declaring external C functions with `extern "C"`
- exposing Rust functions for C callers
- converting data layouts carefully across the boundary

Important caveats:

- use `#[repr(C)]` for shared struct layouts
- do not pass Rust-native types like `String`, `Vec<T>`, or references directly across a C ABI boundary unless you are intentionally building a matching contract
- C++ interop is not the same thing as "just use `extern \"C\"`"; name mangling, layout, exceptions, and ownership semantics all matter

This area matters particularly for C++ programmers working near systems boundaries, but the key discipline is the same for everyone: keep the unsafe boundary narrow.

## What Rust Still Checks

Inside `unsafe`, Rust still checks many ordinary things:

- types must still line up
- ownership rules outside the unsafe operation still exist
- you do not get permission to write arbitrary nonsense

What changes is that certain low-level operations become allowed without proof from the compiler.

## Common Mistakes

- Treating `unsafe` as a way to silence the borrow checker
- Writing unsafe code without documenting invariants
- Letting unsafe details leak into wide public APIs

## Recap

Unsafe Rust exists for low-level control, not convenience. The standard is simple: if a future maintainer cannot quickly audit the unsafe block and understand its contract, the design is too loose.

## Exercises

1. Write a tiny `unsafe` example and document the safety assumptions.
2. Wrap unsafe logic behind a safe API.
3. Bind to a trivial C function or export a Rust function with a C ABI.
4. List the invariants your wrapper depends on.

## Exit Criteria

- You can explain what Rust still checks inside `unsafe`.
- You treat safety comments as part of the implementation, not optional prose.
