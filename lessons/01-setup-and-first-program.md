# Part 1: Setup and First Program

## Goal

Install Rust, learn the standard toolchain, and understand the basic workflow for building and running a Rust project.

## Mental Model

Rust development is built around a single default toolchain:

- `rustup` installs and updates Rust toolchains
- `rustc` is the compiler
- `cargo` is the build tool, package manager, test runner, and project task runner

If you come from C++, `cargo` fills the role usually split across a compiler, build system, package manager, and test runner. If you come from Python, the biggest shift is that your program is compiled before it runs.

## Install and Verify

Typical commands:

```bash
rustup update
rustc --version
cargo --version
```

If those succeed, your environment is ready.

## Your First Project

Create a new project:

```bash
cargo new hello_rust
cd hello_rust
```

Cargo creates:

- `Cargo.toml`: project metadata and dependencies
- `src/main.rs`: the default binary entry point

The default program looks like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

Run it:

```bash
cargo run
```

## What Cargo Actually Does

Common commands:

```bash
cargo build
cargo run
cargo test
cargo fmt
cargo clippy
```

What they mean:

- `build`: compile the project
- `run`: compile if needed, then run
- `test`: compile tests and execute them
- `fmt`: format code using Rust style conventions
- `clippy`: run lints for common mistakes and non-idiomatic code

This workflow is important because idiomatic Rust development relies heavily on quick compile-check-fix cycles.

## Example: Personalize the Program

```rust
fn main() {
    println!("My name is Dana.");
    println!("I have written C++, Python, and now I am learning Rust.");
}
```

Even this tiny program introduces two useful facts:

- Rust uses functions and blocks with braces like C++
- macros such as `println!` end with `!`

## C++ and Python Notes

For C++ programmers:

- you usually will not invoke `rustc` directly for normal projects
- dependency and test workflows are much more standardized

For Python programmers:

- there is a compile step, but Cargo hides most of the complexity
- the default project structure is more explicit than a loose script directory

## Common Mistakes

- Editing files outside `src/` and expecting Cargo to discover them automatically
- Forgetting that `println!` is a macro, not a regular function
- Treating compiler errors as unusual; in Rust, they are normal feedback

## Recap

At the end of this part, you should be comfortable creating a project and running the standard Cargo workflow without hunting for commands.

## Exercises

1. Create a project named `hello_rust`.
2. Print your name and prior programming background.
3. Add a second print line describing why you want to learn Rust.
4. Run `cargo build`, `cargo run`, and `cargo test`.

## Exit Criteria

- You can explain the roles of `rustup`, `cargo`, and `rustc`.
- You can create and run a new project without looking up the commands.
