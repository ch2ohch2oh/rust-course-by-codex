/*
Exercise 16: Async Rust Basics

Tasks:
1. Add an async runtime such as Tokio.
2. Turn one function into `async fn`.
3. Await two tasks concurrently.
4. Compare this design with a thread-based version.

This example uses Tokio to show how async can reduce wall-clock time for
I/O-shaped waiting work.
*/

use std::time::Instant;
use tokio::time::{sleep, Duration};

const FETCH_DELAY: Duration = Duration::from_millis(500);

fn simulated_fetch_sync(name: &str) -> String {
    std::thread::sleep(FETCH_DELAY);
    format!("fetched: {name}")
}

fn fetch_pair_sync() -> (String, String) {
    (
        simulated_fetch_sync("lesson-16"),
        simulated_fetch_sync("lesson-notes"),
    )
}

async fn simulated_fetch(name: &str) -> String {
    sleep(FETCH_DELAY).await;
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
    let sync_start = Instant::now();
    let (sync_lesson, sync_notes) = fetch_pair_sync();
    let sync_elapsed = sync_start.elapsed();

    println!("sync sequential:");
    println!("  {sync_lesson}");
    println!("  {sync_notes}");
    println!("  elapsed: {sync_elapsed:.2?}");

    let async_start = Instant::now();
    let (lesson, notes) = fetch_pair().await;
    let async_elapsed = async_start.elapsed();

    println!("\nasync concurrent:");
    println!("  {lesson}");
    println!("  {notes}");
    println!("  elapsed: {async_elapsed:.2?}");
    println!(
        "\nFor waiting work, async lets both fetches make progress during the same wall-clock window."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn simulated_fetch_returns_labeled_result() {
        assert_eq!(simulated_fetch("lesson-16").await, "fetched: lesson-16");
    }

    #[test]
    fn sync_fetch_pair_returns_both_results() {
        let (lesson, notes) = fetch_pair_sync();
        assert_eq!(lesson, "fetched: lesson-16");
        assert_eq!(notes, "fetched: lesson-notes");
    }

    #[tokio::test]
    async fn fetch_pair_awaits_two_tasks_concurrently() {
        let (lesson, notes) = fetch_pair().await;
        assert_eq!(lesson, "fetched: lesson-16");
        assert_eq!(notes, "fetched: lesson-notes");
    }
}
