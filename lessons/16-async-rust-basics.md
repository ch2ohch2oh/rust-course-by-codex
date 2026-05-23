# Part 16: Async Rust Basics

## Goal

Understand what async Rust is for, how futures and `await` work, and when async is a good fit compared with threads.

## Mental Model

Async Rust is mainly for managing many I/O-bound tasks efficiently. It is not a magic speed button for CPU-heavy work.

Key idea:

- a future represents work that may complete later
- futures are lazy until polled by a runtime
- `.await` yields control until the future makes progress

## `async fn`

```rust
async fn hello() -> String {
    String::from("hello")
}
```

Calling `hello()` does not run it immediately in the usual sense. It creates a future.

To make progress, you need an async runtime such as Tokio.

## Why a Runtime Exists

The runtime:

- polls futures
- schedules tasks
- integrates with timers, sockets, and other async I/O

Without a runtime, most async code has no engine to drive it.

## Example with Tokio

```rust
async fn fetch_name() -> String {
    String::from("rust")
}

#[tokio::main]
async fn main() {
    let name = fetch_name().await;
    println!("{name}");
}
```

This is deliberately simple. The important part is the structure:

- an async function returns a future
- `main` is async under the runtime
- `.await` waits for completion without blocking the whole runtime thread in the same way a regular blocking call would

Two sequential `.await` calls are still sequential. To drive multiple futures concurrently, you need a combinator or task mechanism such as:

- `tokio::join!`
- `tokio::try_join!`
- `tokio::spawn`

That distinction is essential: async syntax alone does not create concurrency.

## Concurrency Versus Parallelism

Async usually provides concurrency:

- many tasks can make progress while waiting on I/O

Threads can provide parallelism:

- multiple CPU cores can execute work at the same time

Some runtimes combine both, but the conceptual distinction matters.

## When Async Helps

Good fit:

- network servers
- clients making many requests
- apps coordinating timers, sockets, and external I/O

Poor fit:

- simple scripts
- CPU-heavy numeric work
- codebases where async infection would overwhelm the benefit

## Common Mistakes

- Assuming async is always faster than threads
- Using blocking I/O inside async tasks
- Adopting async for small programs that do not need it

## Recap

Async Rust is a targeted tool for high-concurrency I/O workloads. Use it when the workload actually benefits from cooperative scheduling.

## Exercises

1. Create a small async program that waits on multiple tasks.
2. Fetch data from two sources concurrently.
3. Compare an async design with a thread-based design.
4. Identify where async helps and where it adds complexity.

## Exit Criteria

- You can explain what a runtime does.
- You know that async is mostly for I/O-bound concurrency, not general speedups.
