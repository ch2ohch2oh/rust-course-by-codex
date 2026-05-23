# Rust Course for C++ and Python Programmers

This repo is a 20-part Rust course plus one supplemental lesson aimed at programmers coming from C++ and Python.
Each part includes:

- a lesson in [`lessons/`](./lessons)
- a hands-on Cargo exercise in [`exercises/`](./exercises)
- a progress checkbox so you can track study status

## How To Use This Course

1. Read the lesson for the part.
2. Work through the matching exercise directory.
3. Build a small artifact for each part, even if the exercise looks simple.
4. Mark the checkbox when you are satisfied with your understanding.

## Progress Tracker

- [x] Part 1: Setup and First Program
- [x] Part 2: Variables, Mutability, and Basic Types
- [x] Part 3: Ownership and Moves
- [ ] Part 4: Borrowing and References
- [ ] Part 5: Strings and Slices
- [ ] Part 6: Control Flow and Pattern Matching
- [ ] Part 7: Structs, Enums, and Methods
- [ ] Part 8: Collections and Iterators
- [ ] Part 9: Error Handling
- [ ] Part 10: Modules, Crates, and Project Structure
- [ ] Part 10A: Rust Attributes
- [ ] Part 11: Traits and Generics
- [ ] Part 12: Lifetimes
- [ ] Part 13: Smart Pointers and Heap Data
- [ ] Part 14: Closures and Functional Style
- [ ] Part 15: Concurrency with Threads and Channels
- [ ] Part 16: Async Rust Basics
- [ ] Part 17: Testing and Documentation
- [ ] Part 18: Unsafe Rust and FFI
- [ ] Part 19: Performance and Idiomatic Rust
- [ ] Part 20: Final Project

## Course Map

### Foundations

- [Part 1](./lessons/01-setup-and-first-program.md)
- [Part 2](./lessons/02-variables-mutability-and-basic-types.md)
- [Part 3](./lessons/03-ownership-and-moves.md)
- [Part 4](./lessons/04-borrowing-and-references.md)
- [Part 5](./lessons/05-strings-and-slices.md)
- [Part 6](./lessons/06-control-flow-and-pattern-matching.md)

### Core Rust

- [Part 7](./lessons/07-structs-enums-and-methods.md)
- [Part 8](./lessons/08-collections-and-iterators.md)
- [Part 9](./lessons/09-error-handling.md)
- [Part 10](./lessons/10-modules-crates-and-project-structure.md)
- [Part 10A](./lessons/10a-rust-attributes.md)
- [Part 11](./lessons/11-traits-and-generics.md)
- [Part 12](./lessons/12-lifetimes.md)

### Intermediate Systems Work

- [Part 13](./lessons/13-smart-pointers-and-heap-data.md)
- [Part 14](./lessons/14-closures-and-functional-style.md)
- [Part 15](./lessons/15-concurrency-with-threads-and-channels.md)
- [Part 16](./lessons/16-async-rust-basics.md)
- [Part 17](./lessons/17-testing-and-documentation.md)

### Advanced Topics

- [Part 18](./lessons/18-unsafe-rust-and-ffi.md)
- [Part 19](./lessons/19-performance-and-idiomatic-rust.md)
- [Part 20](./lessons/20-final-project.md)

## Suggested Pace

- Week 1: Parts 1-3
- Week 2: Parts 4-6
- Week 3: Parts 7-9
- Week 4: Parts 10, 10A-12
- Week 5: Parts 13-15
- Week 6: Parts 16-17
- Week 7: Parts 18-19
- Week 8: Part 20

## Recommended Local Workflow

Use Cargo for everything unless a lesson explicitly says otherwise.

```bash
cd exercises/01-setup-and-first-program
cargo run
cargo test
cargo fmt
cargo clippy
```

## Exercise Layout

Each exercise lives in its own numbered Cargo project, for example:

- [`exercises/01-setup-and-first-program`](./exercises/01-setup-and-first-program)
- [`exercises/03-ownership-and-moves`](./exercises/03-ownership-and-moves)
- [`exercises/10a-rust-attributes`](./exercises/10a-rust-attributes)
- [`exercises/15-concurrency-with-threads-and-channels`](./exercises/15-concurrency-with-threads-and-channels)

The instructions are embedded in code comments and TODO markers to avoid duplicating the same exercise in prose and in starter code.

Use `cargo test` inside each exercise directory to check your work. Some crates now include direct behavior tests; others can only validate part of the exercise because design tasks like module boundaries or final-project scope are not fully automatable.

## Notes For C++ Programmers

- Rust gives memory safety without a GC, but the model is stricter than RAII alone.
- Inheritance is not the center of design; traits are.
- Move semantics matter, but Rust checks far more at compile time.

## Notes For Python Programmers

- Rust is compiled, statically typed, and explicit about ownership.
- `Option` and `Result` replace many `None` and exception-heavy patterns.
- Iterator pipelines feel familiar, but types and lifetimes are enforced.
