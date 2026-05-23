/*
Exercise 16: Async Rust Basics

Tasks:
1. Add an async runtime such as Tokio.
2. Turn one function into `async fn`.
3. Await two tasks concurrently.
4. Compare this design with a thread-based version.

This starter stays dependency-free so the project compiles before you add a runtime.
*/

use tokio::time::{sleep, Duration};

async fn simulated_fetch(name: &str) -> String {
    sleep(Duration::from_millis(10)).await;
    format!("fetched: {name}")
}

async fn fetch_pair() -> (String, String) {
    tokio::join!(
        simulated_fetch("lesson-16"),
        simulated_fetch("lesson-notes")
    )
}

#[tokio::main]
async fn main() {
    let (lesson, notes) = fetch_pair().await;
    println!("{lesson}");
    println!("{notes}");
    println!("Tokio runs many lightweight tasks on a small pool of OS threads.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn simulated_fetch_returns_labeled_result() {
        assert_eq!(simulated_fetch("lesson-16").await, "fetched: lesson-16");
    }

    #[tokio::test]
    async fn fetch_pair_awaits_two_tasks_concurrently() {
        let (lesson, notes) = fetch_pair().await;
        assert_eq!(lesson, "fetched: lesson-16");
        assert_eq!(notes, "fetched: lesson-notes");
    }
}
