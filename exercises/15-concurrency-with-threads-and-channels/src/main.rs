/*
Exercise 15: Concurrency with Threads and Channels

Tasks:
1. Spawn at least two worker threads.
2. Send a message from each worker over a channel.
3. Add a shared counter with `Arc<Mutex<_>>`.
4. Write a short note comparing the message-passing and shared-state approaches.
*/

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn run_workers(worker_count: usize) -> (Vec<String>, usize) {
    let (tx, rx) = mpsc::channel();
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for worker_id in 0..worker_count {
        let tx = tx.clone();
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            {
                let mut count = counter.lock().unwrap();
                *count += 1;
            }

            tx.send(format!("worker {worker_id} finished")).unwrap();
        });
        handles.push(handle);
    }

    drop(tx);

    let mut messages = Vec::new();
    for message in rx {
        messages.push(message);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let counter_value = *counter.lock().unwrap();
    (messages, counter_value)
}

fn main() {
    let (messages, counter) = run_workers(2);
    for message in messages {
        println!("{message}");
    }
    println!("counter = {counter}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workers_send_one_message_each_and_increment_counter() {
        let (messages, counter) = run_workers(3);
        assert_eq!(messages.len(), 3);
        assert_eq!(counter, 3);
    }
}
