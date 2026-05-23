# Part 20: Final Project

## Goal

Use the course material to build one real Rust project that forces you to make ownership, API, error-handling, and concurrency decisions intentionally.

## Mental Model

The final project is where translated habits become visible. A beginner Rust project often looks like C++ or Python written with Rust syntax. The goal here is different: build something that actually uses Rust's strengths.

That means:

- clear ownership boundaries
- explicit error handling
- types that model real states
- minimal unnecessary cloning
- a small, testable public surface

## Project Options

- CLI task manager
- mini HTTP server
- file indexer and search tool
- concurrent log processor
- expression evaluator

Pick a project that is large enough to require structure but small enough to finish.

## Suggested Process

### 1. Define Scope First

Write down:

- what the program does
- what it will not do
- the minimum successful version

This matters because first Rust projects often fail from scope creep, not language difficulty.

### 2. Design the Data Model

Before writing much code, define:

- structs for stored data
- enums for program state and commands
- function signatures that show ownership clearly

### 3. Keep the Core Logic Testable

Do not bury everything in `main`.

A good structure is often:

- `main.rs` for CLI or runtime setup
- `lib.rs` for reusable core logic
- feature modules for parsing, processing, and output

### 4. Add Error Handling Early

Do not wait until the end to replace `unwrap`.

Decide:

- which failures are user-facing
- which are internal bugs
- where `Result` should be propagated

### 5. Add One Serious Refactor Pass

After the program works:

- remove obvious clones
- rename weak types
- tighten visibility
- make enum states more honest
- improve tests

This step matters. Many important Rust design improvements only become obvious after the first working version.

## What Reviewers Should See

A good final project should show:

- idiomatic ownership and borrowing
- sensible module boundaries
- meaningful tests
- readable control flow
- deliberate use of traits, generics, threads, or async where appropriate

It should not look like:

- one giant `main.rs`
- panic-driven control flow
- everything owned, cloned, and public

## Common Mistakes

- Picking a project that is too large
- Introducing async or threads without a clear need
- Over-abstracting before the concrete design is stable

## Recap

The final project is less about feature count and more about design quality. A smaller project with clean Rust decisions is better than a larger project held together by clones, unwraps, and vague types.

## Exercises

1. Write a one-page project plan.
2. Define the module structure before implementation.
3. Build a milestone version with core functionality only.
4. Add tests, documentation, and one refactor pass for API cleanliness.

## Exit Criteria

- You can explain the ownership model of your design.
- You can defend your error-handling and concurrency choices.
- The project feels like Rust rather than translated C++ or Python.
