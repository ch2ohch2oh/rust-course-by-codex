# Part 15: Concurrency with Threads and Channels

## Goal

Use Rust's standard concurrency tools to run work in parallel without losing track of ownership and safety.

## Mental Model

Rust takes concurrency seriously because memory safety bugs are especially dangerous across threads. The type system forces thread-related ownership and borrowing rules to be explicit.

The standard library gives you two common styles:

- message passing with channels
- shared state with synchronization primitives

## Spawning Threads

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("from thread");
    });

    handle.join().unwrap();
}
```

`join` waits for the thread to finish.

## Moving Data Into Threads

Thread closures usually need `move`:

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

The closure takes ownership because the thread may outlive the scope that created it.

In practice, thread closures and captured values also need to satisfy `Send`, and `thread::spawn` usually forces a `'static` lifetime requirement on captured data. That is why borrowing a local variable into a spawned thread often fails even when the logic looks harmless.

## Channels

Channels let threads send messages safely:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("done")).unwrap();
    });

    let message = rx.recv().unwrap();
    println!("{message}");
}
```

This style reduces shared mutable state and can simplify reasoning.

## Shared State with `Arc<Mutex<T>>`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}
```

Why both types?

- `Arc<T>` gives shared ownership across threads
- `Mutex<T>` gives synchronized mutable access

## Message Passing Versus Shared State

Prefer channels when:

- work can be expressed as independent tasks producing results
- you want clearer ownership flow

Prefer shared state when:

- many workers need coordinated access to the same state
- modeling everything as messages becomes awkward

Neither is automatically better. Choose the simpler design for the problem.

## Common Risks

- deadlocks from locking multiple mutexes in inconsistent order
- contention from over-sharing one lock
- moving too much data into threads unnecessarily

Rust prevents data races, but it does not prevent bad architecture.

## Recap

Rust makes thread safety explicit. The right question is not just "can this run in parallel?" but "what ownership model makes the parallel design sane?"

## Exercises

1. Spawn multiple worker threads and join them.
2. Send messages from workers to a collector through a channel.
3. Protect a shared counter with `Arc<Mutex<_>>`.
4. Compare message passing with shared-state design.

## Exit Criteria

- You can explain why the compiler rejects some thread captures.
- You can identify basic deadlock and contention risks.
