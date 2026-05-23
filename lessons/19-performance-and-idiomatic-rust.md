# Part 19: Performance and Idiomatic Rust

## Goal

Learn how to write Rust that performs well without sacrificing clarity or turning every program into a micro-optimization exercise.

## Mental Model

Rust gives you strong performance potential because:

- it compiles to native code
- ownership reduces many runtime costs
- abstractions are often zero-cost after optimization

But potential is not the same as actual performance. Measure first.

## Common Performance Themes

### Avoid Unnecessary Allocation

Prefer borrowing when ownership is not required.

Less ideal:

```rust
fn greet(name: String) {
    println!("hello {name}");
}
```

Better:

```rust
fn greet(name: &str) {
    println!("hello {name}");
}
```

### Avoid Unnecessary Cloning

Cloning can hide design issues. Sometimes it is correct, but it should not be the reflexive fix for ownership friction.

### Use Iterators Without Fear

Iterator-heavy Rust is often optimized very well. Do not assume a plain loop is always faster without evidence.

## Zero-Cost Abstractions

Rust aims for abstractions that disappear after compilation. Traits, iterators, and enums are frequently optimized into efficient machine code.

That does not mean all abstractions are free. It means you should not abandon clarity just because the code looks high-level.

## Measure Before You Optimize

Use tools such as:

- a benchmarking crate such as Criterion
- `cargo clippy`
- profilers for CPU and memory behavior

For quick timings, use optimized builds such as `cargo run --release` or `cargo test --release` before drawing conclusions. Debug-build timings are often misleading in Rust.

Ask practical questions:

- where are the allocations?
- where is time actually spent?
- is the bottleneck algorithmic, architectural, or local?

## Idiomatic Performance Work

Good performance improvements often look like:

- passing `&str` instead of `String`
- reusing buffers
- choosing better data structures
- avoiding repeated parsing
- reducing lock contention in concurrent code

Bad performance work often looks like:

- rewriting clear code into dense code before measuring
- micro-optimizing one function while ignoring an algorithmic bottleneck
- keeping tricky unsafe code with no measurable payoff

## C++ and Python Notes

For C++ programmers:

- Rust gives you a comparable performance ceiling for many workloads, but the idioms differ
- many optimizations come from better ownership and API design rather than pointer-level tweaking

For Python programmers:

- performance wins often come simply from static typing and compiled execution
- but Rust still rewards attention to allocations and data movement

## Common Mistakes

- Optimizing on intuition alone
- Cloning to "make the compiler happy"
- Treating warnings from `clippy` as noise when they point to real simplifications

## Recap

Idiomatic Rust performance work is mostly about clean ownership, good data structures, and measured decisions. The fastest code that no one can safely maintain is usually a bad trade.

## Exercises

1. Reduce allocations in a string-heavy program.
2. Compare two implementations and benchmark them.
3. Remove unnecessary clones from a data-processing pipeline.
4. Run `clippy` and justify any warning you keep.

## Exit Criteria

- You optimize based on evidence, not instinct.
- You can distinguish idiomatic simplification from micro-optimization noise.
