/*
Exercise 16: Async Rust Basics

Tasks:
1. Add an async runtime such as Tokio.
2. Turn one function into `async fn`.
3. Await two tasks concurrently.
4. Compare this design with a thread-based version.

This starter stays dependency-free so the project compiles before you add a runtime.
*/

fn simulated_fetch(name: &str) -> String {
    format!("fetched: {name}")
}

fn main() {
    println!("{}", simulated_fetch("lesson-16"));
    println!("TODO: convert this exercise to async with a runtime.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulated_fetch_returns_labeled_result() {
        assert_eq!(simulated_fetch("lesson-16"), "fetched: lesson-16");
    }
}
