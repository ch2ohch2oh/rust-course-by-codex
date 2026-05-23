/*
Exercise 01: Setup and First Program

Tasks:
1. Change the printed output so it includes your name.
2. Add a second line describing your background in C++ or Python.
3. Add a third line explaining why you are learning Rust.
4. Run `cargo run`, `cargo test`, `cargo fmt`, and `cargo clippy`.
*/

fn main() {
    println!("Hello from Rust, I'm Codex.");
    println!("I spend most of my time in Python and TypeScript, and I'm learning Rust to sharpen my systems programming instincts.");
    println!("Rust is worth learning because it makes performance and correctness feel like part of the same design conversation.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(2 + 2, 4);
    }
}
