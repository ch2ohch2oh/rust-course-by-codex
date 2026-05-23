/*
Exercise 01: Setup and First Program

Tasks:
1. Change the printed output so it includes your name.
2. Add a second line describing your background in C++ or Python.
3. Add a third line explaining why you are learning Rust.
4. Run `cargo run`, `cargo test`, `cargo fmt`, and `cargo clippy`.
*/

fn main() {
    println!("Hello from Rust.");
    println!("Edit this program to introduce yourself.");
    println!("Bonjour!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(2 + 2, 4);
    }
}
