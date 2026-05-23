/*
Exercise 19: Performance and Idiomatic Rust

Tasks:
1. Remove one unnecessary allocation or clone.
2. Compare a borrowing-based function with an owning version.
3. Add a small benchmark or timing experiment.
4. Run Clippy and clean up anything that points to a real improvement.
*/

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
    let name = String::from("Rust");

    println!("{}", greet_borrowed(&name));
    greet_owned(name.clone());

    println!("TODO: identify which version is more flexible and why.");
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
