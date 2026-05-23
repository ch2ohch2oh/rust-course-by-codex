/*
Exercise 19: Performance and Idiomatic Rust

Tasks:
1. Remove one unnecessary allocation or clone.
2. Compare a borrowing-based function with an owning version.
3. Add a small benchmark or timing experiment.
4. Run Clippy and clean up anything that points to a real improvement.
*/

use std::time::Instant;

fn greet_owned(name: String) {
    println!("hello {name}");
}

fn greet_owned_message(name: String) -> String {
    format!("hello {name}")
}

fn greet_borrowed(name: &str) -> String {
    format!("hello {name}")
}

fn main() {
    let name = "Rust";
    let started = Instant::now();

    println!("{}", greet_borrowed(name));
    println!("{}", greet_owned_message(String::from(name)));
    greet_owned(String::from(name));

    println!(
        "borrowed APIs are usually more flexible because callers can pass either `&str` or `&String` without giving up ownership"
    );
    println!("demo elapsed = {:?}", started.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed_and_owned_versions_produce_same_message() {
        let name = String::from("Rust");
        assert_eq!(greet_borrowed(&name), "hello Rust");
        assert_eq!(greet_owned_message(name), "hello Rust");
    }
}
